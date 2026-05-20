//! Integration tests for the [`Validator`] trait contract.

use swe_edge_runtime_scheduler::{validate, Validator};

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

/// @covers: validate
#[test]
fn test_validate_returns_ok_for_valid_impl() {
    assert!(validate(&AlwaysValid).is_ok());
}

/// @covers: validate
#[test]
fn test_validate_returns_err_for_invalid_impl() {
    assert!(validate(&AlwaysInvalid).is_err());
}

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use swe_edge_runtime_scheduler::{validate, TokioSchedulerConfig};

    /// @covers: validate
    #[test]
    fn test_validate_tokio_scheduler_config_default_is_valid() {
        assert!(validate(&TokioSchedulerConfig::default()).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_tokio_scheduler_config_small_stack_is_invalid() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(32),
            ..Default::default()
        };
        assert!(validate(&cfg).is_err());
    }
}
