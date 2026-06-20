#!/usr/bin/env bash
set -euo pipefail
cargo test --workspace
cargo test --workspace --features subprocess
cargo test --workspace --features cli
cargo test --workspace --features message-broker
cargo test --workspace --features observability
cargo test --workspace --features http
cargo test --workspace --features grpc
cargo audit --deny unsound --deny yanked
