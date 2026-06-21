//! Integration tests for the [`Validator`] trait and [`NoopValidator`].
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{CliError, NoopValidator, Validator};

// ─── validate ───────────────────────────────────────────────────────────────

#[test]
fn test_validate_noop_returns_ok_happy() {
    let v = NoopValidator::create();
    assert!(v.validate().is_ok());
}

#[test]
fn test_validate_always_fail_impl_returns_error() {
    struct AlwaysFail;
    impl Validator for AlwaysFail {
        fn validate(&self) -> Result<(), CliError> {
            Err(CliError::InvalidArgs("bad config".to_string()))
        }
    }
    let result = AlwaysFail.validate();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::InvalidArgs(_)));
}

#[test]
fn test_validate_noop_idempotent_edge() {
    let v = NoopValidator::create();
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}
