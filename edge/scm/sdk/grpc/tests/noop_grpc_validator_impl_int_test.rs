//! Tests for the `NoopGrpcValidator` implementation (core/noop).
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::{NoopGrpcValidator, Validator};

/// @covers: NoopGrpcValidator::validate — always returns Ok
#[test]
fn test_noop_validator_validate_always_ok_happy() {
    let v = NoopGrpcValidator;
    assert!(v.validate().is_ok());
}

/// @covers: NoopGrpcValidator::validate — calling twice is idempotent
#[test]
fn test_noop_validator_validate_idempotent_edge() {
    let v = NoopGrpcValidator;
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}

/// @covers: NoopGrpcValidator::validate — no error path via create() (documents absence)
#[test]
fn test_noop_validator_via_create_error() {
    let v = NoopGrpcValidator::create();
    let result = v.validate();
    assert!(
        result.is_ok(),
        "NoopGrpcValidator should never return an error"
    );
}
