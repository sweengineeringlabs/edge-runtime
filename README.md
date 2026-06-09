# swe-edge-runtime

> **TLDR:** Process-level runtime for swe-edge — `RuntimeBuilder` wires HTTP/gRPC ingress, TLS, bearer auth, egress, lifecycle, and Prometheus metrics into a deployable server in one `serve()` call. See [Overview](scm/docs/README.md) for details.

Multi-feature runtime workspace. Two independent crates:

| Feature | Package | Purpose |
|---------|---------|---------|
| `server` | `swe-edge-runtime-server` | Process-level runtime — wires ingress, proxy, domain, egress, lifecycle |
| `message-broker` | `swe-edge-runtime-message-broker` | MessageBroker trait with in-memory, NATS, and Kafka backends |

## Usage

```toml
# server only
swe-edge-runtime-server = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", rev = "<sha>" }

# message-broker: in-memory backend (no external deps)
swe-edge-runtime-message-broker = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", rev = "<sha>", features = ["tokio-rt"] }

# message-broker: NATS backend
swe-edge-runtime-message-broker = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", rev = "<sha>", features = ["nats"] }

# message-broker: Kafka backend (requires cmake and a C compiler)
swe-edge-runtime-message-broker = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", rev = "<sha>", features = ["kafka"] }
```

## Development

Run the bootstrap script once after cloning to install the git hooks and
pre-fetch dependencies:

```bash
# macOS / Linux
bash scm/bootstrap.sh
```

```powershell
# Windows
pwsh scm/bootstrap.ps1
```

This sets `core.hooksPath` to `scm/scripts/hooks`, activating two guards:

| Hook | What it enforces |
|------|-----------------|
| `commit-msg` | Rejects AI-attribution lines (`Co-Authored-By`, `Generated with`, 🤖) |
| `pre-commit` | Runs `cargo fmt`, `cargo clippy -D warnings`, `arch audit`, `cargo audit` |
