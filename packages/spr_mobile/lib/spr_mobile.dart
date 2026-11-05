library;

export 'src/http_transport.dart' show HttpTransport, TransportResponseLost;
export 'src/init.dart';
export 'src/wallet.dart';
export 'src/rust/frb_generated.dart' show RustLib;
// The native wallet is reached through SprWallet, which serializes its
// operations, closes it and makes a native transport from a SprTransport.
export 'src/rust/third_party/spr_mobile.dart'
    hide
        MobileWallet,
        ProvingKeys,
        Transport,
        TransportFailure,
        TransportOutcome,
        derivationMessage;
