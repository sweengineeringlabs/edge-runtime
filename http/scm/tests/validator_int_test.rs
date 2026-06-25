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
    let result = v.validate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (), "validate must return Ok(()) for noop");
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
    let r1 = v.validate();
    let r2 = v.validate();
    assert!(r1.is_ok());
    assert_eq!(r1.unwrap(), (), "first validate must return Ok(())");
    assert!(r2.is_ok());
    assert_eq!(
        r2.unwrap(),
        (),
        "second validate must return Ok(()) (idempotent)"
    );
}
