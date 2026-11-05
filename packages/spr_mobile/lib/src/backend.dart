// The hook [SprWallet.open] and [SprWallet.openWithKeys] open the native
// wallet through. The package library does not export this file: only tests
// replace the backend.
import 'dart:typed_data';

import 'rust/third_party/spr_mobile.dart' as native;

/// A transport as the native wallet calls it: a failure is a message, never
/// a throw.
typedef NativeTransport = Future<native.TransportOutcome> Function(
  native.TransportRequest request,
);

/// The native calls that open a wallet; replaced in tests.
abstract interface class WalletBackend {
  Future<Uint8List> derivationMessage(String solanaPubkey);
  Future<native.MobileWallet> open(
    native.WalletConfig config,
    String solanaPubkey,
    Uint8List derivationSignature,
    NativeTransport transport,
  );
  Future<native.MobileWallet> openWithKeys(
    native.WalletConfig config,
    String solanaPubkey,
    native.WalletKeys keys,
    NativeTransport transport,
  );
}

/// The native library's own calls.
class NativeBackend implements WalletBackend {
  const NativeBackend();

  @override
  Future<Uint8List> derivationMessage(String solanaPubkey) =>
      native.derivationMessage(solanaPubkey: solanaPubkey);

  @override
  Future<native.MobileWallet> open(
    native.WalletConfig config,
    String solanaPubkey,
    Uint8List derivationSignature,
    NativeTransport transport,
  ) async => native.MobileWallet.open(
    config: config,
    solanaPubkey: solanaPubkey,
    derivationSignature: derivationSignature,
    transport: await native.Transport.newInstance(send: transport),
  );

  @override
  Future<native.MobileWallet> openWithKeys(
    native.WalletConfig config,
    String solanaPubkey,
    native.WalletKeys keys,
    NativeTransport transport,
  ) async => native.MobileWallet.openWithKeys(
    config: config,
    solanaPubkey: solanaPubkey,
    keys: keys,
    // A native transport per wallet: opening a wallet consumes it.
    transport: await native.Transport.newInstance(send: transport),
  );
}
