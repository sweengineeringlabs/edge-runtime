//! [`Validator`] impl for [`TokioSchedulerConfig`] (spi layer).

use crate::api::types::TokioSchedulerConfig;
use crate::api::validator::Validator;

impl Validator for TokioSchedulerConfig {
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

    #[test]
    fn test_validate_returns_ok_for_default_config() {
        assert!(TokioSchedulerConfig::default().validate().is_ok());
    }

    #[test]
    fn test_validate_returns_err_for_stack_below_minimum() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(32),
            ..Default::default()
        };
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn test_validate_returns_ok_for_valid_stack_size() {
        let cfg = TokioSchedulerConfig {
            thread_stack_kib: Some(512),
            ..Default::default()
        };
        assert!(cfg.validate().is_ok());
    }
}
