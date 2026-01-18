#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building nessus-launcher (library)"
cargo build --manifest-path "$ROOT_DIR/Cargo.toml"

echo "==> Building nessus-cli (binary)"
cargo build --manifest-path "$ROOT_DIR/nessus-cli/Cargo.toml"

echo "==> Done"

