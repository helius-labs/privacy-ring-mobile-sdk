#!/usr/bin/env bash
set -euo pipefail

# Stages the example app's proving assets: the 2 -> 2 transfer's zopro key and
# the fixture requests. The key downloads from the proving keys' CDN (or
# copies from $ZOPRO_KEYS_DIR when set) and is checked against the wallet's
# lockfile. The native proof job also stages a merge key in the cache.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
asset_dir="$repo_root/packages/spr_mobile/example/assets/proving"
key_name="transfer_confidential_2_2.zopro"
# The lockfile's prefix; its last part names the key set's directory, as the
# wallet lays out its key directory.
key_prefix="proving-keys/78751f5d1c591777/"
key_dir="$repo_root/.cache/proving/$(basename "$key_prefix")"
key_checksum="4687bf25f01c6a26d3e1f6e6ea2dc9935e6b9751f935b54927292d1808e839b9"
request_checksum="38f39f6ac8c23d4d74105bf9dd743213a94470cc46219667d67fd67db1941df3"

sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{ print $1 }'
  else
    sha256sum "$1" | awk '{ print $1 }'
  fi
}

verify() {
  local path="$1"
  local expected="$2"
  if [[ "$(sha256 "$path")" != "$expected" ]]; then
    echo "checksum mismatch: $path" >&2
    exit 1
  fi
}

stage_key() (
  local name="$1" checksum="$2"
  local destination="$key_dir/$name"
  if [[ -f "$destination" ]] && [[ "$(sha256 "$destination")" == "$checksum" ]]; then
    return
  fi
  mkdir -p "$key_dir"
  local temporary_key
  temporary_key="$(mktemp "$key_dir/.zopro-key.XXXXXX")"
  trap 'rm -f "$temporary_key"' EXIT
  if [[ -n "${ZOPRO_KEYS_DIR:-}" ]]; then
    cp "$ZOPRO_KEYS_DIR/$name" "$temporary_key"
  else
    local base_url="${ZOLANA_PROVING_KEYS_URL:-https://d3gbdb0egjwcw9.cloudfront.net}"
    echo "downloading $name"
    curl -fsSL "$base_url/$key_prefix$name" -o "$temporary_key"
  fi
  verify "$temporary_key" "$checksum"
  mv "$temporary_key" "$destination"
)

stage_key "$key_name" "$key_checksum"
if [[ "${1:-}" == "--with-merge-key" ]]; then
  stage_key merge_8_1.zopro 768f1c8bb8d3667edc77c17124ccc094aedbbcc774eaefccd5f9ccb216e7d791
fi
verify "$repo_root/fixtures/prove-request-2x2.json" "$request_checksum"

mkdir -p "$asset_dir"
cp "$key_dir/$key_name" "$asset_dir/$key_name"
# The fixture requests of every shape, for the proof timing harness.
cp "$repo_root"/fixtures/prove-request-*.json "$asset_dir/"
echo "staged demo proving assets in $asset_dir"
