//! Tests for the `NoopValidator` implementation (core/noop).
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{NoopValidator, Validator};

#[test]
fn test_noop_validator_validate_always_ok_happy() {
    let v = NoopValidator;
    let result = v.validate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (), "validate must return unit on success");
}

#[test]
fn test_noop_validator_validate_idempotent_edge() {
    let v = NoopValidator;
    let r1 = v.validate();
    let r2 = v.validate();
    assert!(r1.is_ok());
    assert_eq!(r1.unwrap(), (), "first validate must return unit");
    assert!(r2.is_ok());
    assert_eq!(
        r2.unwrap(),
        (),
        "second validate must also return unit (idempotent)"
    );
}

#[test]
fn test_noop_validator_via_create_error() {
    // The create() path produces a validator that never errors — documents the absence of error path.
    let v = NoopValidator::create();
    let result = v.validate();
    assert!(result.is_ok(), "NoopValidator should never return an error");
    assert_eq!(
        result.unwrap(),
        (),
        "validate must return Ok(()) not Ok(other)"
    );
}
