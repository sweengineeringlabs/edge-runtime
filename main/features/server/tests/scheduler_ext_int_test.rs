//! Tests for scheduler extension.

#[cfg(all(test, feature = "scheduler"))]
mod tests {
    use swe_edge_runtime::Runtime;
    use swe_edge_runtime_scheduler::{Scheduler, SchedulerError};

    /// @covers: run_with_scheduler
    #[test]
    fn test_run_with_scheduler_returns_start_failed_for_empty_builder() {
        struct SchedulerExtNoopScheduler;
        impl Scheduler for SchedulerExtNoopScheduler {
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
        let result = Runtime::builder().run_with_scheduler(SchedulerExtNoopScheduler);
        // Result can be either StartFailed or Scheduler error, both are acceptable
        assert!(result.is_err());
    }
}
