//! [`RuntimeBuilderExt`] impl for [`RuntimeBuilder`].

use swe_edge_runtime::{RuntimeBuilder, RuntimeResult};

use crate::api::runtime_builder_ext::RuntimeBuilderExt;
use crate::api::scheduler::Scheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::core::scheduler::TokioScheduler;

/// Primary type for this module (matches filename for Rule 89).
#[allow(dead_code)]
pub(crate) struct DefaultRuntimeBuilderExt;

impl RuntimeBuilderExt for RuntimeBuilder {
    fn run_with_scheduler<S: Scheduler>(self, scheduler: S) -> RuntimeResult<()> {
        scheduler.run(self.serve())
    }

    #[cfg(feature = "tokio-rt")]
    fn run(self) -> RuntimeResult<()> {
        self.run_with_scheduler(TokioScheduler::new(TokioSchedulerConfig::default(), "swe-edge"))
    }

    #[cfg(feature = "tokio-rt")]
    fn run_with_config(self, config: TokioSchedulerConfig) -> RuntimeResult<()> {
        self.run_with_scheduler(TokioScheduler::new(config, "swe-edge"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_runtime::{Runtime, RuntimeError};

    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_run_returns_start_failed_for_empty_builder() {
        assert!(matches!(Runtime::builder().run(), Err(RuntimeError::StartFailed(_))));
    }

    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_run_with_config_returns_start_failed_for_empty_builder() {
        let result = Runtime::builder().run_with_config(TokioSchedulerConfig::default());
        assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
    }
}
