# swe-edge-runtime

Multi-feature runtime workspace. Three independent crates:

| Feature | Package | Purpose |
|---------|---------|---------|
| `server` | `swe-edge-runtime-server` | Process-level runtime — wires ingress, proxy, domain, egress, lifecycle |
| `scheduler` | `swe-edge-runtime-scheduler` | Async executor — `Scheduler` trait + `TokioScheduler` |
| `message-broker` | `swe-edge-runtime-message-broker` | MessageBroker trait with in-memory and NATS backends |

## Usage

```toml
# server only
swe-edge-runtime-server = { git = "...", branch = "dev" }

# server + scheduler run() convenience
swe-edge-runtime-server = { git = "...", branch = "dev", features = ["scheduler"] }

# scheduler standalone (no server dependency)
swe-edge-runtime-scheduler = { git = "...", branch = "dev" }

# message-broker standalone
swe-edge-runtime-message-broker = { git = "...", branch = "dev", features = ["tokio-rt"] }
```
