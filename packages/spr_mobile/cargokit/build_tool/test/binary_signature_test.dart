import 'dart:convert';
import 'dart:typed_data';

import 'package:build_tool/src/binary_signature.dart';
import 'package:ed25519_edwards/ed25519_edwards.dart';
import 'package:test/test.dart';

void main() {
  final key = generateKey();
  const hash = '178e18a0f0cb5a472e5cf33dbc542272';
  const otherHash = '57ee18bfc1dd344b212c68d289b0fb3f';
  const fileName = 'aarch64-apple-ios_libmopro_flutter_bindings.a';
  const otherTarget = 'aarch64-apple-ios-sim_libmopro_flutter_bindings.a';
  final binary = Uint8List.fromList(utf8.encode('static library'));

  bool verifies(
    List<int> signature, {
    String crateHash = hash,
    String file = fileName,
    List<int>? data,
  }) =>
      verifyBinary(
        key.publicKey,
        crateHash: crateHash,
        fileName: file,
        binary: data ?? binary,
        signature: signature,
      );

  final signature = signBinary(
    key.privateKey,
    crateHash: hash,
    fileName: fileName,
    binary: binary,
  );

  test('verifies the binary it was signed for', () {
    expect(verifies(signature), isTrue);
  });

  test('refuses the binary under another crate hash', () {
    expect(verifies(signature, crateHash: otherHash), isFalse);
  });

  test('refuses the binary as another target', () {
    expect(verifies(signature, file: otherTarget), isFalse);
  });

  test('refuses other bytes', () {
    expect(verifies(signature, data: utf8.encode('static librarz')), isFalse);
  });

  test('refuses a signature of the bytes alone', () {
    expect(verifies(sign(key.privateKey, binary)), isFalse);
  });

  test('refuses another key and a malformed signature', () {
    final other = signBinary(
      generateKey().privateKey,
      crateHash: hash,
      fileName: fileName,
      binary: binary,
    );
    expect(verifies(other), isFalse);
    expect(verifies(signature.sublist(0, 63)), isFalse);
    expect(verifies(const []), isFalse);
  });
}
