//! `RuntimeBuilder` scheduler extension — bridges server and scheduler crates.
//!
//! Enabled by the `scheduler` feature. Adds synchronous entry points to
//! [`RuntimeBuilder`] so consumers can start a binary with no tokio boilerplate.

#[cfg(feature = "scheduler")]
use swe_edge_runtime_scheduler::Scheduler;

#[cfg(feature = "scheduler")]
use crate::api::error::{RuntimeError, RuntimeResult};
#[cfg(feature = "scheduler")]
use crate::api::runtime::RuntimeBuilder;

#[cfg(feature = "scheduler")]
impl RuntimeBuilder {
    /// Drive the runtime with a custom [`Scheduler`] implementation.
    ///
    /// Blocks the calling thread until the runtime shuts down or an error
    /// occurs. Use this when you bring your own async executor.
    pub fn run_with_scheduler<S: Scheduler>(self, scheduler: S) -> RuntimeResult<()> {
        scheduler
            .run(self.serve())
            .map_err(|e| RuntimeError::Scheduler(e.to_string()))
    }

    /// Drive the runtime with the tokio scheduler and default config.
    pub fn run(self) -> RuntimeResult<()> {
        use swe_edge_runtime_scheduler::{tokio_scheduler, TokioSchedulerConfig};
        let s = tokio_scheduler(TokioSchedulerConfig::default(), "swe-edge");
        self.run_with_scheduler(s)
    }

    /// Drive the runtime with the tokio scheduler and the supplied config.
    pub fn run_with_config(
        self,
        config: swe_edge_runtime_scheduler::TokioSchedulerConfig,
    ) -> RuntimeResult<()> {
        use swe_edge_runtime_scheduler::tokio_scheduler;
        let s = tokio_scheduler(config, "swe-edge");
        self.run_with_scheduler(s)
    }
}

#[cfg(all(test, feature = "scheduler"))]
mod tests {
    use crate::api::error::RuntimeError;
    use crate::api::runtime::Runtime;

    /// @covers: run
    #[test]
    fn test_run_returns_start_failed_for_empty_builder() {
        assert!(matches!(
            Runtime::builder().run(),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    /// @covers: run_with_config
    #[test]
    fn test_run_with_config_returns_start_failed_for_empty_builder() {
        use swe_edge_runtime_scheduler::TokioSchedulerConfig;
        let result = Runtime::builder().run_with_config(TokioSchedulerConfig::default());
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }
}
