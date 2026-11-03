import 'package:flutter_test/flutter_test.dart';
import 'package:zolana_mobile/zolana_mobile.dart';
import 'package:zolana_mobile_demo/prover_policy.dart';

void main() {
  test('the devnet example selects the SDK signed policy', () {
    expect(demoProverNetwork, ProverTeeNetwork.devnet);
    expect(demoProverPolicy, isNull);
  });
}
