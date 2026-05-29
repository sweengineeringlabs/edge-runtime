//! Integration tests for swe-edge-runtime-scheduler dependency coverage.

use swe_edge_runtime_scheduler::{tokio_scheduler, TokioSchedulerConfig};

/// @covers: swe-edge-runtime-scheduler
#[test]
fn test_tokio_scheduler_constructs_with_default_config() {
    let cfg = TokioSchedulerConfig::default();
    let _sched = tokio_scheduler(cfg);
}
