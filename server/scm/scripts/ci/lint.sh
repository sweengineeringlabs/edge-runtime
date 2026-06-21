#!/usr/bin/env bash
set -euo pipefail
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo clippy --workspace --features subprocess -- -D warnings
cargo clippy --workspace --features cli -- -D warnings
cargo clippy --workspace --features message-broker -- -D warnings
cargo clippy --workspace --features observability -- -D warnings
cargo clippy --workspace --features http -- -D warnings
cargo clippy --workspace --features grpc -- -D warnings
