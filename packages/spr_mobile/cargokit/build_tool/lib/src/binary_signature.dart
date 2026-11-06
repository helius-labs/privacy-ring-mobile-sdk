/// Signatures of precompiled binaries.
///
/// A signature covers the crate hash and the file name, which starts with the
/// target triple, together with the SHA-256 of the binary. A signed binary
/// therefore verifies only as that file of the release `precompiled_<hash>`;
/// put under another crate hash or target, it is refused.
library;

import 'dart:convert';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:ed25519_edwards/ed25519_edwards.dart';

Uint8List signBinary(
  PrivateKey privateKey, {
  required String crateHash,
  required String fileName,
  required List<int> binary,
}) =>
    sign(privateKey, _message(crateHash, fileName, binary));

bool verifyBinary(
  PublicKey publicKey, {
  required String crateHash,
  required String fileName,
  required List<int> binary,
  required List<int> signature,
}) =>
    verify(
      publicKey,
      _message(crateHash, fileName, binary),
      Uint8List.fromList(signature),
    );

/// The domain, the crate hash and the file name, each in UTF-8 and followed
/// by a zero byte, then the 32-byte SHA-256 of the binary.
Uint8List _message(String crateHash, String fileName, List<int> binary) {
  final message = BytesBuilder(copy: false);
  for (final field in ['cargokit precompiled binary', crateHash, fileName]) {
    message.add(utf8.encode(field));
    message.addByte(0);
  }
  message.add(sha256.convert(binary).bytes);
  return message.takeBytes();
}
