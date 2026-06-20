#!/usr/bin/env bash
set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/main"
cd "$CRATE_DIR"

echo "==> build"
cargo build

echo "==> build --release"
cargo build --release

echo "==> build OK"
