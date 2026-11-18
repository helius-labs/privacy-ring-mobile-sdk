#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
package="$repo_root/packages/spr_mobile"
consumer="$(mktemp -d "${TMPDIR:-/tmp}/spr-local-prover-consumer.XXXXXX")"
trap 'rm -r "$consumer"' EXIT

mkdir -p "$consumer/src"
cp -R "$package/native/spr-local-prover" "$consumer/prover"
cp "$repo_root/scripts/consumer_fixture/local_prover.rs" "$consumer/src/main.rs"
cp "$repo_root/Cargo.lock" "$consumer/Cargo.lock"
cat > "$consumer/Cargo.toml" <<'TOML'
[package]
name = "spr-local-prover-consumer"
version = "0.0.0"
edition = "2021"

[dependencies]
spr-local-prover = { path = "prover" }
TOML

cargo run --manifest-path "$consumer/Cargo.toml" -- \
  "$repo_root/fixtures/prove-request-2x2.json" \
  "$package/example/assets/proving/transfer_confidential_2_2.zopro" \
  "$consumer/cache"
