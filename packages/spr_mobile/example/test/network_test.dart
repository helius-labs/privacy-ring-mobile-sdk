import 'package:flutter_test/flutter_test.dart';
import 'package:spr_mobile/spr_mobile.dart';
import 'package:zolana_mobile_demo/demo_assets.dart';
import 'package:zolana_mobile_demo/network.dart';

void main() {
  test('the demo selects the signed devnet policy only with a prover', () {
    final local = demoConfig(provingKeyDir: '/keys');
    expect(local.proverUrl, isNull);
    expect(local.proverTeeNetwork, isNull);
    expect(local.proverTeePolicy, isNull);
    expect(local.mints, [for (final asset in demoAssets) ?asset.config]);

    final remote = demoConfig(
      provingKeyDir: '/keys',
      proverUrl: Network.devnet.proverUrl,
    );
    expect(remote.proverTeeNetwork, ProverTeeNetwork.devnet);
    expect(remote.proverTeePolicy, isNull);
  });
}
