#!/usr/bin/env bash
set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/main"
cd "$CRATE_DIR"

echo "==> fmt check"
cargo fmt --check

echo "==> clippy"
cargo clippy -- -D warnings

echo "==> lint OK"
