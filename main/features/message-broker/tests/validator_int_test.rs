//! Tests for the [`Validator`] trait contract.

use swe_edge_runtime_message_broker::Validator;

#[test]
fn test_custom_validator_ok_path() {
    struct AlwaysOk;
    impl Validator for AlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }
    assert!(AlwaysOk.validate().is_ok());
}

#[test]
fn test_custom_validator_error_path() {
    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always invalid".into())
        }
    }
    let result = AlwaysErr.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "always invalid");
}
