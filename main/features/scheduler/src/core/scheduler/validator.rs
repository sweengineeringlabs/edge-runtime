//! [`Validator`] impl for [`TokioSchedulerConfig`].

use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;

/// Primary type for this module (matches filename for Rule 89).
#[allow(dead_code)]
pub(crate) struct Validator;

impl crate::api::traits::Validator for TokioSchedulerConfig {
    fn validate(&self) -> Result<(), String> {
        if let Some(stack_kib) = self.thread_stack_kib {
            if stack_kib < 64 {
                return Err(format!(
                    "thread_stack_kib must be at least 64 KiB, got {stack_kib}"
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::Validator as ValidatorTrait;

    #[test]
    fn test_validate_returns_ok_for_default_config() {
        assert!(ValidatorTrait::validate(&TokioSchedulerConfig::default()).is_ok());
    }

    #[test]
    fn test_validate_returns_err_for_stack_below_minimum() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(32),
            ..Default::default()
        };
        assert!(ValidatorTrait::validate(&cfg).is_err());
    }

    #[test]
    fn test_validate_returns_ok_for_valid_stack_size() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(512),
            ..Default::default()
        };
        assert!(ValidatorTrait::validate(&cfg).is_ok());
    }
}
