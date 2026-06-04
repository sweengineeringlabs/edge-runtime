# Edge Scheduler

Runtime-agnostic async scheduler for `swe-edge-runtime`. Ships a tokio-backed
`TokioScheduler` by default; bring your own runtime by implementing `Scheduler`.
