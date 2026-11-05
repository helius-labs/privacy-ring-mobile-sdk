import 'package:spr_mobile/spr_mobile.dart';

/// The application's own transport: the package's [HttpTransport] behind a
/// log of what it sent. Pass [send] as the `transport` of `SprWallet.open`.
class AppTransport {
  final _http = HttpTransport();

  /// Method and path of each request sent; the query, which holds the API
  /// key, is left out.
  final sent = <String>[];

  Future<TransportResponse> send(TransportRequest request) {
    sent.add('${request.method} ${Uri.parse(request.url).path}');
    return _http.send(request);
  }

  void close() => _http.close();
}
