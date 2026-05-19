//! SAF — scheduler public factory surface.

#[cfg(feature = "tokio-rt")]
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
use crate::api::scheduler::Scheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::traits::Validator;
#[cfg(feature = "tokio-rt")]
use crate::spi::TokioScheduler;

/// Validate a value that implements [`Validator`].
#[cfg(feature = "tokio-rt")]
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

/// Construct a tokio-backed scheduler with the given config and thread name prefix.
///
/// Use the returned scheduler with [`Scheduler::run`] to drive any async future.
#[cfg(feature = "tokio-rt")]
pub fn tokio_scheduler(
    config: TokioSchedulerConfig,
    thread_name: impl Into<String>,
) -> impl Scheduler {
    TokioScheduler::new(config, thread_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_validate_returns_ok_for_valid_config() {
        assert!(validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: tokio_scheduler
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_tokio_scheduler_factory_produces_working_scheduler() {
        let s = tokio_scheduler(TokioSchedulerConfig::default(), "test");
        let result: Result<(), _> = s.run(async {});
        assert!(result.is_ok());
    }
}
