# swe-edge-runtime-scheduler — Architecture

## Layers

| Layer | Path | Purpose |
|---|---|---|
| `api/` | `src/api/` | `Scheduler` trait, `TokioScheduler`, `TokioSchedulerConfig`, `RuntimeBuilderExt` |
| `core/` | `src/core/` | `impl Scheduler for TokioScheduler` — tokio runtime construction, panic hook |
| `saf/` | `src/saf/` | `run()`, `run_with_config()`, `run_with_scheduler()` free functions |

## Feature flags

| Feature | Default | Enables |
|---|---|---|
| `tokio-rt` | yes | `TokioScheduler`, `TokioSchedulerConfig`, `run()`, `run_with_config()` |
