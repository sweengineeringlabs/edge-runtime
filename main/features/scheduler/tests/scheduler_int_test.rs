//! Integration tests for the [`Scheduler`] trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_scheduler::{Scheduler, SchedulerError, SchedulerSvc, TokioSchedulerConfig};

struct OkScheduler;
impl Scheduler for OkScheduler {
    fn run<F, T>(&self, fut: F) -> Result<T, SchedulerError>
    where
        F: std::future::Future<Output = T> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| SchedulerError::StartFailed(e.to_string()))
            .map(|rt| rt.block_on(fut))
    }
}

/// @covers: Scheduler::run
#[test]
fn test_scheduler_runs_future_and_returns_ok() {
    let s = OkScheduler;
    assert!(s.run(async { Ok::<(), ()>(()) }).is_ok());
}

/// @covers: Scheduler::run
#[test]
fn test_scheduler_passes_through_future_output() {
    let s = OkScheduler;
    let result: Result<i32, SchedulerError> = s.run(async { 42 });
    assert_eq!(result.unwrap(), 42);
}

/// @covers: SchedulerSvc::tokio_scheduler
#[cfg(feature = "tokio-rt")]
#[test]
fn test_tokio_scheduler_factory_produces_working_scheduler() {
    use swe_edge_runtime_scheduler::Scheduler;
    let s = SchedulerSvc::tokio_scheduler(TokioSchedulerConfig::default(), "test");
    let result: Result<(), _> = s.run(async {});
    assert!(result.is_ok());
}
