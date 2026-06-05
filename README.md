# swe-edge-runtime

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
