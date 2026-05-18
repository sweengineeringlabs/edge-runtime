# Runtime Architecture

## Workspace overview

The runtime workspace is a single Rust crate — `swe-edge-runtime` — that wires all other
workspaces into a production-ready daemon. It is the only crate consumers need as a direct
dependency; it re-exports the full `ingress` and `egress` public surfaces.

| Crate | Package | Purpose |
|-------|---------|---------|
| `runtime` | `swe-edge-runtime` | Production daemon — lifecycle, config, signals, observability |

---

## SEA module layout

```
src/
├── api/
│   ├── runtime_config.rs    # RuntimeConfig — ports, timeouts, feature flags
│   ├── runtime_manager.rs   # RuntimeManager — holds all wired components
│   ├── runtime_error.rs     # RuntimeError — startup and shutdown errors
│   └── traits.rs            # SEA interface contract
├── core/
│   ├── runtime_manager.rs   # DefaultRuntimeManager implementation
│   └── config_loader.rs     # TOML config resolution
├── saf/
│   └── mod.rs               # run(), runtime_manager(), Runtime::builder(), load_config()
└── lib.rs                   # pub use saf::*; re-exports ingress + egress
```

---

## Startup sequence

```
main()
  │
  ▼
load_config()          — resolves config/default.toml → application.toml → env vars
  │
  ▼
Runtime::builder()     — assemble ingress, egress, lifecycle, observability
  │
  ▼
RuntimeManager::start() — bind ports, spawn transport tasks
  │
  ▼
await SIGTERM / SIGINT
  │
  ▼
RuntimeManager::shutdown() — drain in-flight requests, release resources
```

---

## Config resolution order

```
config/default.toml          (SWE defaults — committed)
        ↓  overridden by
application.toml             (workspace-level — committed)
        ↓  overridden by
tenant override              (runtime-injected)
        ↓  overridden by
environment variables        (deployment-injected, highest precedence)
```

---

## Key exports

| Export | Purpose |
|--------|---------|
| `run(config, ingress, egress, lifecycle)` | Block until signal then drain — simple entry point |
| `Runtime::builder()` | Incremental assembly with type-safe builder pattern |
| `load_config()` | Load `RuntimeConfig` from the standard config layer |
| `init_tracing()` | Initialise `tracing-subscriber` (requires `observability` feature) |

## Feature flags

| Flag | Enables |
|------|---------|
| `observability` | `init_tracing()`, `observe_lifecycle_monitor()` |

---

## See Also

- [Architecture Overview](../../docs/3-architecture/architecture.md)
- [Ingress Architecture](../../ingress/docs/3-design/architecture.md)
- [Egress Architecture](../../egress/docs/3-design/architecture.md)
- [Proxy Architecture](../../proxy/docs/architecture.md)
- [Domain Architecture](../../domain/docs/architecture.md)
- [Config Architecture](../../config/swe-edge-config/docs/architecture.md)
- [Observability Config Architecture](../../observ-config/swe-edge-observ-config/docs/architecture.md)
- [Developer Guide](../../docs/4-development/developer_guide.md)
