//! Tests for the `NoopValidator` implementation (core/noop).
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{NoopValidator, Validator};

#[test]
fn test_noop_validator_validate_always_ok_happy() {
    let v = NoopValidator;
    assert!(v.validate().is_ok());
}

#[test]
fn test_noop_validator_validate_idempotent_edge() {
    let v = NoopValidator;
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}

#[test]
fn test_noop_validator_via_create_error() {
    // The create() path produces a validator that never errors — documents the absence of error path.
    let v = NoopValidator::create();
    let result = v.validate();
    assert!(result.is_ok(), "NoopValidator should never return an error");
}
