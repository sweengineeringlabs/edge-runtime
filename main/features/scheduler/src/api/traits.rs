//! Core interface contracts for `swe-edge-runtime-scheduler`.

#[allow(unused_imports)]
pub use crate::api::scheduler::Scheduler;

/// Validates that a value is in a legal state before use.
#[allow(dead_code)]
pub trait Validator {
    /// Return `Ok(())` when the value is valid, or `Err` with an actionable
    /// description of the first validation failure.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_ok_path() {
        struct AlwaysValid;
        impl Validator for AlwaysValid {
            fn validate(&self) -> Result<(), String> {
                Ok(())
            }
        }
        assert!(AlwaysValid.validate().is_ok());
    }

    #[test]
    fn test_validator_err_path() {
        struct AlwaysInvalid;
        impl Validator for AlwaysInvalid {
            fn validate(&self) -> Result<(), String> {
                Err("invalid".into())
            }
        }
        assert!(AlwaysInvalid.validate().is_err());
    }
}
