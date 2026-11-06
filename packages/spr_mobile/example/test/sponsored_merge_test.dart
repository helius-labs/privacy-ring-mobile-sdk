import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:cryptography/cryptography.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:spr_mobile/spr_mobile.dart';
import 'package:spr_mobile/src/rust/frb_generated.dart' show RustLibApi;
import 'package:zolana_mobile_demo/sponsored_merge.dart';
import 'package:zolana_mobile_demo/spend_checks.dart';
import 'package:zolana_mobile_demo/encoding.dart';

void main() {
  setUpAll(() => RustLib.initMock(api: _Api()));

  test(
    'proxy auth is scoped, ciphertext is preserved, redirects are not followed',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final seen = <({String path, String? auth, List<int> body})>[];
      server.listen((request) async {
        final body = await request.fold<List<int>>(
          [],
          (bytes, chunk) => bytes..addAll(chunk),
        );
        seen.add((
          path: request.uri.path,
          auth: request.headers.value('x-sf-auth'),
          body: body,
        ));
        if (request.uri.path.endsWith('/redirect')) {
          request.response.statusCode = 302;
          request.response.headers.set('location', '/unexpected');
        } else {
          request.response.headers.set('spr-tee', '1');
          request.response.add([8, 9, 10]);
        }
        await request.response.close();
      });
      final backend = SponsoredBackend(
        Uri.parse('http://127.0.0.1:${server.port}/'),
        'example-auth',
      );
      addTearDown(() async {
        backend.close();
        await server.close(force: true);
      });
      Future<TransportResponse> send(String path) => backend.transport(
        TransportRequest(
          method: 'POST',
          url: 'http://127.0.0.1:${server.port}$path',
          headers: {'spr-tee': '1', 'spr-tee-enc': 'opaque', 'x-sync': 'true'},
          body: Uint8List.fromList([1, 2, 3]),
          timeoutMs: 1000,
        ),
      );
      final proof = await send('/v1/spr/prove/merge_8_1');
      expect(proof.body, [8, 9, 10]);
      expect(proof.headers['spr-tee'], '1');
      await send('/rpc');
      final redirect = await send('/v1/spr/redirect');
      expect(redirect.status, 302);
      expect(seen.map((r) => r.auth), ['example-auth', null, 'example-auth']);
      expect(seen.every((r) => r.body.join(',') == '1,2,3'), isTrue);
      expect(seen.any((r) => r.path == '/unexpected'), isFalse);
    },
  );

  test(
    'queued status GET keeps auth, encrypted headers and encoded job ID',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      server.listen((request) async {
        expect(request.method, 'GET');
        expect(
          request.uri.toString(),
          '/v1/spr/prove/merge_24_1/status?jobId=job%2F1',
        );
        expect(request.headers.value('x-sf-auth'), 'example-auth');
        expect(request.headers.value('spr-tee-enc'), 'enc');
        expect(request.headers.value('spr-tee-ciphertext'), '010203');
        expect(
          await request.fold<List<int>>([], (all, chunk) => all..addAll(chunk)),
          isEmpty,
        );
        request.response.headers.set('spr-tee', 'v1');
        request.response.add([4, 5, 6]);
        await request.response.close();
      });
      final backend = SponsoredBackend(
        Uri.parse('http://127.0.0.1:${server.port}/'),
        'example-auth',
      );
      addTearDown(() async {
        backend.close();
        await server.close(force: true);
      });
      final response = await backend.transport(
        TransportRequest(
          method: 'GET',
          url: '${backend.proverUrl}/prove/merge_24_1/status?jobId=job%2F1',
          headers: {
            'spr-tee': 'v1',
            'spr-tee-enc': 'enc',
            'spr-tee-ciphertext': '010203',
          },
          body: Uint8List(0),
          timeoutMs: 1000,
        ),
      );
      expect(response.status, 200);
      expect(response.headers['spr-tee'], 'v1');
      expect(response.body, [4, 5, 6]);
    },
  );

  test('merge retries persist state before every relayer handoff', () async {
    final store = _Store();
    final backend = _Backend();
    final wallet = _Wallet();
    final flow = SponsoredFlow(
      wallet,
      _UnusedSigner(),
      backend,
      store,
      _UnusedChecks(),
    );
    await flow.start(rounds: 1, treeId: 0);
    store.failWrites = true;
    for (var retry = 0; retry < 2; retry++) {
      await expectLater(flow.mergeRounds(), throwsStateError);
      expect(backend.relays, 0);
    }
    store.failWrites = false;
    await flow.mergeRounds();
    expect(wallet.prepares, 1);
    expect(backend.relays, 1);
    expect(store.data!['phase'], 'ready');
  });

  test('recovery resends saved bytes and retains state until completion is durable', () async {
    for (final replayFails in [false, true]) {
      final store = _Store()
        ..data = {
          'version': 1,
          'backend': 'https://backend.example/',
          'phase': 'submitted',
          'merges': [],
          'payment': 'payment-snapshot',
          'paymentSignature': base58Encode(List.filled(64, 7)),
        };
      final wallet = _Wallet()
        ..unknown = true
        ..replayFails = replayFails;
      final flow = SponsoredFlow(
        wallet,
        _UnusedSigner(),
        _Backend(),
        store,
        _UnusedChecks(),
      );
      await flow.restore();
      store.failCompletion = true;
      await expectLater(flow.reconcilePayment(), throwsStateError);
      expect(flow.complete, isFalse);
      expect(store.data!['phase'], 'submitted');
      expect(wallet.submissions.single, List.filled(64, 7));
      expect(wallet.prepares, 0);
      store.failCompletion = false;
      await flow.reconcilePayment();
      expect(flow.complete, isTrue);
      expect(store.data!['phase'], 'complete');
    }
  });

  test('run state survives restart and rejects tampered ciphertext', () async {
    final directory = await Directory.systemTemp.createTemp(
      'sponsored-state-test',
    );
    addTearDown(() => directory.delete(recursive: true));
    final file = File('${directory.path}/run');
    final key = SecretKey(List.filled(32, 7));
    final store = SponsoredStore(file, key);
    final state = {
      'phase': 'submitted',
      'paymentSignature': 'saved-before-submission',
      'amount': '123456789',
    };
    await store.write(state);
    expect(await SponsoredStore(file, key).read(), state);
    expect(
      utf8.decode(await file.readAsBytes(), allowMalformed: true),
      isNot(contains('saved-before-submission')),
    );
    final bytes = await file.readAsBytes();
    bytes[20] ^= 1;
    await file.writeAsBytes(bytes);
    await expectLater(
      store.read(),
      throwsA(isA<SecretBoxAuthenticationError>()),
    );
  });

  test(
    'a refused payment returns to approval and can be signed again',
    () async {
      for (final refusal in [
        const WalletError.notesAlreadySpent(),
        const WalletError.client(message: 'simulation failed'),
      ]) {
        final store = _Store()..data = _approvalRun();
        final wallet = _Wallet()..submitError = refusal;
        final flow = SponsoredFlow(
          wallet,
          _Signer(),
          _Backend(),
          store,
          _PassingChecks(),
        );
        await flow.restore();
        await expectLater(
          flow.signAndSend((_) async => true),
          throwsA(refusal),
        );
        expect(flow.paymentSignature, isNull);
        expect(store.data!['phase'], 'approval');
        expect(store.data!.containsKey('paymentSignature'), isFalse);
        wallet.submitError = null;
        expect(
          await flow.signAndSend((_) async => true),
          base58Encode(List.filled(64, 7)),
        );
        expect(store.data!['phase'], 'complete');
        expect(wallet.submissions, hasLength(2));
      }
    },
  );

  test('an open outcome keeps the signed payment submitted', () async {
    for (final open in [
      WalletError.transactionOutcomeUnknown(
        signature: base58Encode(List.filled(64, 7)),
        message: 'not found',
      ),
      const WalletError.walletClosed(),
    ]) {
      final store = _Store()..data = _approvalRun();
      final wallet = _Wallet()..submitError = open;
      final flow = SponsoredFlow(
        wallet,
        _Signer(),
        _Backend(),
        store,
        _PassingChecks(),
      );
      await flow.restore();
      await expectLater(flow.signAndSend((_) async => true), throwsA(open));
      expect(store.data!['phase'], 'submitted');
      expect(store.data!['paymentSignature'], base58Encode(List.filled(64, 7)));
    }
  });

  test('recovery returns a payment the chain refused to approval', () async {
    final store = _Store()
      ..data = {
        ..._approvalRun(),
        'phase': 'submitted',
        'paymentSignature': base58Encode(List.filled(64, 7)),
      };
    final wallet = _Wallet()
      ..confirmError = const WalletError.notesAlreadySpent();
    final flow = SponsoredFlow(
      wallet,
      _Signer(),
      _Backend(),
      store,
      _PassingChecks(),
    );
    await flow.restore();
    await expectLater(
      flow.reconcilePayment(),
      throwsA(const WalletError.notesAlreadySpent()),
    );
    expect(store.data!['phase'], 'approval');
    expect(store.data!.containsKey('paymentSignature'), isFalse);
    expect(wallet.submissions, isEmpty);
  });

  test(
    'plans by tree before requesting a run and avoids singleton rounds',
    () async {
      final wallet = _Wallet();
      final backend = _Backend();
      final store = _Store();
      final flow = SponsoredFlow(
        wallet,
        _UnusedSigner(),
        backend,
        store,
        _UnusedChecks(),
      );
      wallet.notes = [
        for (var i = 0; i < 4; i++)
          SpendableNote(
            id: 'note-$i',
            mint: null,
            amount: BigInt.from(10),
            treeId: i ~/ 2,
            mergeable: true,
          ),
      ];
      await expectLater(flow.start(rounds: 2, treeId: 0), throwsStateError);
      expect(backend.runs, 0);
      wallet.notes = [
        for (var i = 0; i < 5; i++)
          SpendableNote(
            id: 'note-$i',
            mint: null,
            amount: BigInt.from(10),
            treeId: 0,
            mergeable: true,
          ),
        SpendableNote(
          id: 'data',
          mint: null,
          amount: BigInt.one,
          treeId: 0,
          mergeable: false,
        ),
        SpendableNote(
          id: 'other-mint',
          mint: 'token',
          amount: BigInt.one,
          treeId: 0,
          mergeable: true,
        ),
        SpendableNote(
          id: 'other-tree',
          mint: null,
          amount: BigInt.one,
          treeId: 1,
          mergeable: true,
        ),
      ];
      await flow.start(rounds: 2, treeId: 0, maxInputs: 4);
      expect(store.data!['inputIds'], [
        ['note-0', 'note-1', 'note-2'],
        ['note-3', 'note-4'],
      ]);
    },
  );

  test('a saved plan survives new arrivals, restart and retry without reselection', () async {
    final store = _Store();
    final backend = _Backend()..failSecondRelay = true;
    final wallet = _Wallet();
    var flow = SponsoredFlow(
      wallet,
      _UnusedSigner(),
      backend,
      store,
      _UnusedChecks(),
    );
    await flow.start(rounds: 2, treeId: 0, maxInputs: 2);
    final plan = jsonDecode(jsonEncode(store.data!['inputIds']));
    wallet.notes.insert(
      0,
      SpendableNote(
        id: 'new-small-note',
        mint: null,
        amount: BigInt.one,
        treeId: 0,
        mergeable: true,
      ),
    );
    // Restart before preparation: none of the planned inputs are reserved yet.
    flow = SponsoredFlow(
      wallet,
      _UnusedSigner(),
      backend,
      store,
      _UnusedChecks(),
    );
    await flow.restore();
    await expectLater(flow.mergeRounds(), throwsStateError);
    expect(wallet.selections, plan);
    expect(store.data!['inputIds'], plan);
    // Restart after an ambiguous handoff: reuse prepared bytes, not a new proof.
    backend.failSecondRelay = false;
    flow = SponsoredFlow(
      wallet,
      _UnusedSigner(),
      backend,
      store,
      _UnusedChecks(),
    );
    await flow.restore();
    await flow.mergeRounds();
    expect(wallet.prepares, 2);
    expect(flow.phase, 'ready');
    expect(store.data!['inputIds'], plan);
  });
}

/// A run whose payment is proved and saved, waiting for the user's approval.
Map<String, dynamic> _approvalRun() => {
  'version': 1,
  'backend': 'https://backend.example/',
  'run': {
    'id': 'run',
    'slots': 1,
    'target': {
      'sponsor': 'sponsor',
      'nonce': 1,
      'treeId': 0,
      'expiresAt': 2000000000,
      'maxPriorityFeeLamports': 10000,
      'computeUnitLimit': 400000,
    },
  },
  'mint': null,
  'merges': [],
  'phase': 'approval',
  'payment': 'payment-snapshot',
  'requestedOutputs': [
    {'kind': 'public', 'recipient': 'recipient', 'amount': '5'},
  ],
};

class _Store implements SponsoredStore {
  Map<String, dynamic>? data;
  bool failWrites = false, failCompletion = false;
  @override
  Future<Map<String, dynamic>?> read() async => data == null
      ? null
      : jsonDecode(jsonEncode(data)) as Map<String, dynamic>;
  @override
  Future<void> write(Map<String, dynamic> state) async {
    if (failWrites || (failCompletion && state['phase'] == 'complete')) {
      throw StateError('disk full');
    }
    data = jsonDecode(jsonEncode(state)) as Map<String, dynamic>;
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Backend implements SponsoredBackend {
  int relays = 0, runs = 0;
  bool failSecondRelay = false;
  @override
  Uri get base => Uri.parse('https://backend.example/');
  @override
  Future<Map<String, dynamic>> call(
    String path, [
    Map<String, dynamic>? body,
  ]) async {
    if (path == 'runs') {
      runs++;
      return {
        'id': 'run',
        'slots': body!['slots'],
        'target': {
          'sponsor': 'sponsor',
          'nonce': 1,
          'treeId': body['treeId'],
          'expiresAt': 2000000000,
          'maxPriorityFeeLamports': 10000,
          'computeUnitLimit': 400000,
        },
      };
    }
    expect(path, 'runs/run/merges');
    if (failSecondRelay && body!['slot'] == 1) {
      throw StateError('lost response');
    }
    relays++;
    return {'signature': 'merge-signature'};
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Wallet implements SprWallet {
  int prepares = 0;
  bool unknown = false, replayFails = false;
  Object? submitError, confirmError;
  final submissions = <List<int>>[];
  final selections = <List<String>>[];
  List<SpendableNote> notes = [
    for (var i = 0; i < 4; i++)
      SpendableNote(
        id: 'note-$i',
        mint: null,
        amount: BigInt.from(10 + i),
        treeId: 0,
        mergeable: true,
      ),
  ];
  @override
  Future<List<SpendableNote>> spendableNotes() async => notes;
  @override
  String get solanaPublicKey => 'owner';
  @override
  Future<PreparedTransaction> prepareMerge({
    String? mint,
    int? maxInputs,
    String? feePayer,
    Proving? proving,
    BigInt? priorityFeeLamports,
    CacheTarget? cacheTarget,
    List<String>? inputIds,
  }) async {
    prepares++;
    expect(mint, isNull);
    expect(maxInputs, isNull);
    selections.add(List.of(inputIds!));
    return _Transaction();
  }

  @override
  Future<String> exportPreparedTransaction(
    PreparedTransaction transaction,
  ) async => 'snapshot';
  @override
  Future<PreparedTransaction> restorePreparedTransaction(
    String snapshot,
  ) async => _Transaction();
  @override
  Future<void> verifyCachedOutputs(List<CachedOutput> outputs) async {}
  @override
  Future<void> confirm(
    PreparedTransaction transaction,
    String signature,
  ) async {
    if (unknown) {
      unknown = false;
      throw WalletError.transactionOutcomeUnknown(
        signature: signature,
        message: 'not found',
      );
    }
    if (confirmError case final error?) throw error;
  }

  @override
  Future<String> submit(
    PreparedTransaction transaction,
    List<Uint8List> signatures,
  ) async {
    expect(transaction.message, [1, 2, 3]);
    submissions.add(signatures.single);
    if (replayFails) throw const WalletError.notesAlreadySpent();
    if (submitError case final error?) throw error;
    return base58Encode(signatures.single);
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Transaction implements PreparedTransaction {
  @override
  Uint8List get message => Uint8List.fromList([1, 2, 3]);
  @override
  BigInt get priorityFeeLamports => BigInt.from(5000);
  @override
  List<OutputOpening> get outputs => const [];
  @override
  CachedOutput get cachedOutput => _CachedOutput();
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _CachedOutput implements CachedOutput {
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Signer implements SolanaSigner {
  @override
  String get publicKey => 'owner';
  @override
  Future<Uint8List> signMessage(
    Uint8List message, {
    PreparedTransaction? transaction,
  }) async => Uint8List.fromList(List.filled(64, 7));
}

class _PassingChecks implements SpendChecks {
  @override
  Future<List<String>> verify({
    required Uint8List message,
    required List<OutputOpening> openings,
    required String sender,
    required SpendRequest request,
  }) async => const [];

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _Api implements RustLibApi {
  @override
  Future<String> sprMobileCacheTargetCacheAddress({
    required CacheTarget that,
  }) async => 'cache';

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _UnusedSigner implements SolanaSigner {
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}

class _UnusedChecks implements SpendChecks {
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}
