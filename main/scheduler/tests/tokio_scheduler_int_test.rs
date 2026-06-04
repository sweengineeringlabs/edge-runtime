//! Integration tests for [`SchedulerSvc::tokio_scheduler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_scheduler::{Scheduler, SchedulerSvc, TokioSchedulerConfig};

/// @covers: SchedulerSvc::tokio_scheduler
#[test]
fn test_tokio_scheduler_runs_future_successfully() {
    let scheduler = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
    let result: Result<(), _> = scheduler.run(async {});
    assert!(result.is_ok());
}

/// @covers: SchedulerSvc::tokio_scheduler
#[test]
fn test_tokio_scheduler_passes_through_output() {
    let scheduler = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
    let result: Result<i32, _> = scheduler.run(async { 99 });
    assert_eq!(result.unwrap(), 99);
}

/// @covers: SchedulerSvc::tokio_scheduler
#[test]
fn test_tokio_scheduler_respects_thread_name() {
    use std::num::NonZeroUsize;
    let config = TokioSchedulerConfig {
        workers: NonZeroUsize::new(1),
        ..Default::default()
    };
    let scheduler = SchedulerSvc::tokio_scheduler(config, "swe-test");
    let result: Result<(), _> = scheduler.run(async {});
    assert!(result.is_ok());
}
