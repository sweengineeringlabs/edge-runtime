//! Integration tests for the [`Validator`] trait (rule 222) and `VALIDATOR_SVC` const (rule 221).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_message_broker::{Validator, VALIDATOR_SVC};

struct AlwaysOk;
impl Validator for AlwaysOk {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct AlwaysErr;
impl Validator for AlwaysErr {
    fn validate(&self) -> Result<(), String> {
        Err("configuration is invalid: missing required field".into())
    }
}

// --- Validator::validate (rule 222) ---

/// @covers: Validator::validate
#[test]
fn test_validate_valid_impl_happy() {
    let result = AlwaysOk.validate();
    assert_eq!(result, Ok(()), "valid impl must return Ok(())");
}

/// @covers: Validator::validate
#[test]
fn test_validate_invalid_impl_error() {
    let result = AlwaysErr.validate();
    assert!(result.is_err(), "invalid impl must return Err");
    let err = result.unwrap_err();
    assert_eq!(err, "configuration is invalid: missing required field", "error message must match");
}

/// @covers: Validator::validate
#[test]
fn test_validate_error_message_is_non_empty_edge() {
    let msg = AlwaysErr.validate().unwrap_err();
    assert!(!msg.is_empty(), "error message must not be empty");
}

// --- VALIDATOR_SVC const (rule 221) ---

/// @covers: VALIDATOR_SVC
#[test]
fn test_validator_svc_identifier_is_stable_happy() {
    assert_eq!(VALIDATOR_SVC, "validator");
}
