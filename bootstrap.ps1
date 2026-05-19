# Bootstrap the swe-edge-runtime workspace.
# Builds all three member crates and runs their test suites.
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "==> Building runtime workspace (all features)..."
cargo build --workspace

Write-Host "==> Testing runtime workspace..."
cargo test --workspace

Write-Host "==> Done."
