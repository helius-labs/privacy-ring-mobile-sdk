import 'package:spr_mobile/spr_mobile.dart';

import 'demo_assets.dart';

const _apiKey = String.fromEnvironment('ZOLANA_API_KEY');
const _rpcUrl = String.fromEnvironment('ZOLANA_RPC_URL');
const _indexerUrl = String.fromEnvironment('ZOLANA_INDEXER_URL');
const _policy = String.fromEnvironment('ZOLANA_TEE_POLICY');

/// The Helius gateway's SPR routes on devnet: the indexer and the attested
/// prover.
const _gateway = 'https://beta-devnet.helius-rpc.com/v1/spr?api-key=$_apiKey';

class Network {
  const Network(this.rpcUrl, this.indexerUrl, this.proverUrl);

  final String rpcUrl;
  final String indexerUrl;
  final String proverUrl;

  /// SPR's devnet-e deployment through the Helius gateway, keyed by the
  /// Helius key from `--dart-define=ZOLANA_API_KEY=...`. `ZOLANA_RPC_URL`
  /// and `ZOLANA_INDEXER_URL` replace its Solana RPC and indexer.
  static const devnet = Network(
    _rpcUrl == ''
        ? 'https://beta-devnet.helius-rpc.com/?api-key=$_apiKey'
        : _rpcUrl,
    _indexerUrl == '' ? _gateway : _indexerUrl,
    _gateway,
  );

  /// Whether the build has the Helius key [devnet] needs.
  static const hasApiKey = _apiKey != '';
}

/// The demo's wallet configuration on [network], with its proving keys under
/// [provingKeyDir] and the demo assets. With a [proverUrl] it selects the
/// SDK's signed devnet TEE policy, or the reviewed static policy from
/// `--dart-define=ZOLANA_TEE_POLICY=...`.
WalletConfig demoConfig({
  required String provingKeyDir,
  Network network = Network.devnet,
  String? proverUrl,
  bool? requireProverTee,
  bool? proverSyncOnly,
  Proving? proving,
  int? maxSpendInputs,
}) => WalletConfig(
  rpcUrl: network.rpcUrl,
  indexerUrl: network.indexerUrl,
  provingKeyDir: provingKeyDir,
  proving: proving,
  proverUrl: proverUrl,
  requireProverTee: requireProverTee,
  proverSyncOnly: proverSyncOnly,
  proverTeePolicy: proverUrl == null || _policy == '' ? null : _policy,
  proverTeeNetwork: proverUrl == null || _policy != ''
      ? null
      : ProverTeeNetwork.devnet,
  maxSpendInputs: maxSpendInputs,
  mints: [for (final asset in demoAssets) ?asset.config],
);
