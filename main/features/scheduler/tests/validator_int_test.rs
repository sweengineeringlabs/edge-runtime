//! Integration tests for the [`Validator`] trait contract and [`SchedulerSvc::validate`].

use swe_edge_runtime_scheduler::{SchedulerSvc, Validator};

struct AlwaysValid;
impl Validator for AlwaysValid {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct AlwaysInvalid;
impl Validator for AlwaysInvalid {
    fn validate(&self) -> Result<(), String> {
        Err("always invalid".into())
    }
}

/// @covers: SchedulerSvc::validate
#[test]
fn test_validate_returns_ok_for_valid_impl() {
    assert!(SchedulerSvc::validate(&AlwaysValid).is_ok());
}

/// @covers: SchedulerSvc::validate
#[test]
fn test_validate_returns_err_for_invalid_impl() {
    assert!(SchedulerSvc::validate(&AlwaysInvalid).is_err());
}

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use swe_edge_runtime_scheduler::{SchedulerSvc, TokioSchedulerConfig};

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_validate_tokio_scheduler_config_default_is_valid() {
        assert!(SchedulerSvc::validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: SchedulerSvc::validate
    #[test]
    fn test_validate_tokio_scheduler_config_small_stack_is_invalid() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(32),
            ..Default::default()
        };
        assert!(SchedulerSvc::validate(&cfg).is_err());
    }
}
