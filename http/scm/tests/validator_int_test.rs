//! Integration tests for the `Validator` trait and `NoopValidator`.
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{NoopValidator, Validator};

fn noop() -> NoopValidator {
    NoopValidator
}

// ─── validate ───────────────────────────────────────────────────────────────

#[test]
fn test_validate_noop_happy() {
    let v = noop();
    assert!(v.validate().is_ok());
}

#[test]
fn test_validate_noop_error() {
    // An always-failing validator documents the error path.
    struct AlwaysFail;
    impl Validator for AlwaysFail {
        fn validate(&self) -> Result<(), String> {
            Err("bad config".to_string())
        }
    }
    let v = AlwaysFail;
    let result = v.validate();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "bad config");
}

#[test]
fn test_validate_noop_idempotent_edge() {
    // Edge: calling validate() twice returns Ok both times.
    let v = noop();
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}
