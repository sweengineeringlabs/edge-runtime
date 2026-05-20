#!/usr/bin/env bash
set -euo pipefail

# Bootstrap the swe-edge-runtime workspace.
# Builds all three member crates and runs their test suites.

echo "==> Building runtime workspace (all features)..."
cargo build --workspace

echo "==> Testing runtime workspace..."
cargo test --workspace

echo "==> Done."
