import 'package:spr_mobile/spr_mobile.dart';

const _policy = String.fromEnvironment('ZOLANA_TEE_POLICY');

/// A reviewed static override, otherwise use the SDK's signed devnet policy.
const demoProverPolicy = _policy == '' ? null : _policy;
const demoProverNetwork = _policy == '' ? ProverTeeNetwork.devnet : null;
