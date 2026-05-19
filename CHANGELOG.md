# Changelog

All notable changes to the `swe-edge-runtime` workspace are documented here.

## [Unreleased]

### Added
- Virtual workspace consolidating `server`, `scheduler`, and `message-broker` crates
- `swe-edge-runtime-server` — main process runtime (moved from standalone `runtime/`)
- `swe-edge-runtime-scheduler` — async executor with `Scheduler` trait + `TokioScheduler`
- `swe-edge-runtime-message-broker` — `MessageBroker` trait with in-memory and NATS backends
- `scheduler` feature on server adds `RuntimeBuilder::run()` / `run_with_scheduler()` / `run_with_config()`
- `message-broker` feature on server wires `MessageBroker` into `RuntimeManager` health checks
