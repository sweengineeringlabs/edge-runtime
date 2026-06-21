#!/usr/bin/env bash
set -euo pipefail

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/main"
cd "$CRATE_DIR"

echo "==> build"
cargo build

echo "==> test"
cargo test

echo "==> audit"
cargo audit --deny unsound --deny yanked

echo "==> test OK"
