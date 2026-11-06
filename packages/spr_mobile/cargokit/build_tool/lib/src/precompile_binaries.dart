/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';

import 'package:ed25519_edwards/ed25519_edwards.dart';
import 'package:github/github.dart';
import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'binary_signature.dart';
import 'builder.dart';
import 'cargo.dart';
import 'crate_hash.dart';
import 'download.dart';
import 'options.dart';
import 'rustup.dart';
import 'target.dart';

final _log = Logger('precompile_binaries');

class PrecompileBinaries {
  PrecompileBinaries({
    required this.privateKey,
    required this.githubToken,
    required this.repositorySlug,
    required this.manifestDir,
    required this.targets,
    this.androidSdkLocation,
    this.androidNdkVersion,
    this.androidMinSdkVersion,
    this.tempDir,
  });

  final PrivateKey privateKey;
  final String githubToken;
  final RepositorySlug repositorySlug;
  final String manifestDir;
  final List<Target> targets;
  final String? androidSdkLocation;
  final String? androidNdkVersion;
  final int? androidMinSdkVersion;
  final String? tempDir;

  static String fileName(Target target, String name) {
    return '${target.rust}_$name';
  }

  static String signatureFileName(Target target, String name) {
    return '${target.rust}_$name.sig';
  }

  Future<void> run() async {
    final crateInfo = CrateInfo.load(manifestDir);

    final targets = List.of(this.targets);
    if (targets.isEmpty) {
      targets.addAll([
        ...Target.buildableTargets(),
        if (androidSdkLocation != null) ...Target.androidTargets(),
      ]);
    }

    _log.info('Precompiling binaries for $targets');

    final hash = CrateHash.compute(manifestDir);
    _log.info('Computed crate hash: $hash');

    final String tagName = 'precompiled_$hash';

    final github = GitHub(auth: Authentication.withToken(githubToken));
    final repo = github.repositories;
    final release = await _getOrCreateRelease(
      repo: repo,
      tagName: tagName,
      packageName: crateInfo.packageName,
      hash: hash,
    );

    final tempDir = this.tempDir != null
        ? Directory(this.tempDir!)
        : Directory.systemTemp.createTempSync('precompiled_');

    tempDir.createSync(recursive: true);

    final crateOptions = CargokitCrateOptions.load(
      manifestDir: manifestDir,
    );

    final buildEnvironment = BuildEnvironment(
      configuration: BuildConfiguration.release,
      crateOptions: crateOptions,
      targetTempDir: tempDir.path,
      manifestDir: manifestDir,
      crateInfo: crateInfo,
      isAndroid: androidSdkLocation != null,
      androidSdkPath: androidSdkLocation,
      androidNdkVersion: androidNdkVersion,
      androidMinSdkVersion: androidMinSdkVersion,
    );

    final rustup = Rustup();

    for (final target in targets) {
      final artifactNames = getArtifactNames(
        target: target,
        libraryName: crateInfo.packageName,
        remote: true,
      );

      if (await _isReleased(release, hash, target, artifactNames)) {
        _log.info('All artifacts for $target are released - skipping');
        continue;
      }
      await _deleteAssets(repo, release, target, artifactNames);

      _log.info('Building for $target');

      final builder =
          RustBuilder(target: target, environment: buildEnvironment);
      builder.prepare(rustup);
      final res = await builder.build();

      final assets = <CreateReleaseAsset>[];
      for (final name in artifactNames) {
        final file = File(path.join(res, name));
        if (!file.existsSync()) {
          throw Exception('Missing artifact: ${file.path}');
        }

        final data = file.readAsBytesSync();
        final assetName = PrecompileBinaries.fileName(target, name);
        final create = CreateReleaseAsset(
          name: assetName,
          contentType: "application/octet-stream",
          assetData: data,
        );
        final signature = signBinary(
          privateKey,
          crateHash: hash,
          fileName: assetName,
          binary: data,
        );
        final signatureCreate = CreateReleaseAsset(
          name: signatureFileName(target, name),
          contentType: "application/octet-stream",
          assetData: signature,
        );
        final verified = verifyBinary(
          public(privateKey),
          crateHash: hash,
          fileName: assetName,
          binary: data,
          signature: signature,
        );
        if (!verified) {
          throw Exception('Signature verification failed');
        }
        assets.add(create);
        assets.add(signatureCreate);
      }
      _log.info('Uploading assets: ${assets.map((e) => e.name)}');
      for (final asset in assets) {
        // This seems to be failing on CI so do it one by one
        int retryCount = 0;
        while (true) {
          try {
            await repo.uploadReleaseAssets(release, [asset]);
            break;
          } on Exception catch (e) {
            if (retryCount == 10) {
              rethrow;
            }
            ++retryCount;
            _log.shout(
                'Upload failed (attempt $retryCount, will retry): ${e.toString()}');
            await Future.delayed(Duration(seconds: 2));
          }
        }
      }
    }

    _log.info('Cleaning up');
    tempDir.deleteSync(recursive: true);
  }

  /// Whether the release has every artifact of [target] with a signature of
  /// this key for this crate hash and file name. A file signed for another
  /// crate hash or target, a missing signature or a broken upload makes the
  /// target build again.
  Future<bool> _isReleased(
    Release release,
    String crateHash,
    Target target,
    List<String> artifactNames,
  ) async {
    ReleaseAsset? asset(String name) =>
        release.assets?.where((e) => e.name == name).firstOrNull;
    for (final name in artifactNames) {
      final fileName = PrecompileBinaries.fileName(target, name);
      final binaryAsset = asset(fileName);
      final signatureAsset = asset(signatureFileName(target, name));
      if (binaryAsset == null || signatureAsset == null) return false;
      final binary =
          await getWithRetry(Uri.parse(binaryAsset.browserDownloadUrl!));
      final signature =
          await getWithRetry(Uri.parse(signatureAsset.browserDownloadUrl!));
      if (binary.statusCode != 200 ||
          signature.statusCode != 200 ||
          !verifyBinary(
            public(privateKey),
            crateHash: crateHash,
            fileName: fileName,
            binary: binary.bodyBytes,
            signature: signature.bodyBytes,
          )) {
        _log.info('$fileName has no valid signature - replacing it');
        return false;
      }
    }
    return true;
  }

  /// Deletes the artifacts of [target] and their signatures from the release,
  /// so that the new uploads can take their names.
  Future<void> _deleteAssets(
    RepositoriesService repo,
    Release release,
    Target target,
    List<String> artifactNames,
  ) async {
    final names = {
      for (final name in artifactNames) ...[
        PrecompileBinaries.fileName(target, name),
        signatureFileName(target, name),
      ],
    };
    for (final asset in release.assets ?? <ReleaseAsset>[]) {
      if (names.contains(asset.name)) {
        _log.info('Deleting asset ${asset.name}');
        await repo.deleteReleaseAsset(repositorySlug, asset);
      }
    }
  }

  Future<Release> _getOrCreateRelease({
    required RepositoriesService repo,
    required String tagName,
    required String packageName,
    required String hash,
  }) async {
    Release release;
    try {
      _log.info('Fetching release $tagName');
      release = await repo.getReleaseByTagName(repositorySlug, tagName);
    } on ReleaseNotFound {
      _log.info('Release not found - creating release $tagName');
      try {
        release = await repo.createRelease(
            repositorySlug,
            CreateRelease.from(
              tagName: tagName,
              name: 'Precompiled binaries ${hash.substring(0, 8)}',
              targetCommitish: null,
              isDraft: false,
              isPrerelease: false,
              body: 'Precompiled binaries for crate $packageName, '
                  'crate hash $hash.',
            ));
      } on Exception catch (e) {
        // Jobs for other targets run in parallel and may create it first.
        _log.info('Failed to create release $tagName ($e) - fetching it');
        release = await repo.getReleaseByTagName(repositorySlug, tagName);
      }
    }
    return release;
  }
}
