# swe-edge-runtime

Production daemon runner for the `swe-edge` stack.

Wires ingress (HTTP + gRPC), egress (HTTP + gRPC), lifecycle monitoring, config
loading, and observability into a single entry point. Call `run()` to block until
`SIGTERM`/`SIGINT`; call `runtime_manager()` when you need a handle to drive the
lifecycle yourself.

## Quick start

```rust
use std::sync::Arc;
use swe_edge_runtime::{
    run, Runtime, RuntimeConfig,
    DefaultInput, DefaultOutput,
    new_null_lifecycle_monitor,
};

#[tokio::main]
async fn main() {
    let config   = RuntimeConfig::default();
    let ingress  = Arc::new(DefaultInput::empty());   // swap with real handler
    let egress   = Arc::new(DefaultOutput::empty());  // swap with real client
    let lifecycle = new_null_lifecycle_monitor();

    run(config, ingress, egress, lifecycle).await.unwrap();
}
```

## RuntimeBuilder

Use `Runtime::builder()` for incremental assembly:

```rust
use swe_edge_runtime::Runtime;

let manager = Runtime::builder()
    .with_config(config)
    .with_http_handler(Arc::new(my_handler))
    .with_egress_http(Arc::new(http_client))
    .with_lifecycle(Arc::new(monitor))
    .build()
    .await
    .unwrap();
```

## Public surface (`saf/`)

| Export | Purpose |
|--------|---------|
| `run(config, ingress, egress, lifecycle)` | Block until signal, then drain |
| `runtime_manager(…)` | Assemble a `RuntimeManager` without blocking |
| `Runtime::builder()` | Incremental runtime assembly |
| `load_config()` / `load_config_from()` | TOML config loading |
| `load_tenant_config()` / `load_section()` | Per-tenant and section loaders |
| `observe_lifecycle_monitor(…)` | Wrap a `LifecycleMonitor` with metrics |
| `init_tracing(…)` | Initialise tracing subscriber (feature `observability`) |

## Ingress / egress re-exports

`swe-edge-runtime` re-exports the full `swe-edge-ingress` and `swe-edge-egress`
public surfaces so application crates need only one dependency:

```toml
[dependencies]
swe-edge-runtime = { git = "https://github.com/sweengineeringlabs/edge", tag = "v0.1.0", features = ["observability"] }
```

## Lifecycle

`run()` listens for `SIGTERM` (Unix) or `SIGINT` (all platforms). On signal it:

1. Sends shutdown to the HTTP and gRPC transport tasks
2. Calls `RuntimeManager::shutdown()`
3. Waits up to `config.shutdown_timeout_secs` before returning `ShutdownTimeout`

## Config loading

Layers resolve in order — later layers win:

```
config/default.toml  →  application.toml  →  tenant override  →  env vars
```

All loaders return typed structs deserialized via `serde` + `config-rs`.

## Feature flags

| Flag | Enables |
|------|---------|
| `observability` | `init_tracing()` via `tracing-subscriber` |

## Building

```bash
cd runtime
cargo build
cargo test
cargo clippy -- -D warnings
```
