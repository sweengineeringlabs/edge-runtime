//! Integration tests for `saf::validator_svc` — the SAF validator surface.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::NoopGrpcValidator;

/// @covers: NoopGrpcValidator::create — happy path
#[test]
fn test_create_noop_validator_run_validate_happy() {
    let v = NoopGrpcValidator::create();
    assert!(v.run_validate().is_ok());
}

/// @covers: NoopGrpcValidator::create — no error path exists
#[test]
fn test_create_noop_validator_never_errors_error() {
    let v = NoopGrpcValidator::create();
    let result = v.run_validate();
    assert!(
        result.is_ok(),
        "NoopGrpcValidator::run_validate must never fail"
    );
}

/// @covers: NoopGrpcValidator::create — multiple calls are independent
#[test]
fn test_create_noop_validator_idempotent_edge() {
    let v1 = NoopGrpcValidator::create();
    let v2 = NoopGrpcValidator::create();
    assert!(v1.run_validate().is_ok());
    assert!(v2.run_validate().is_ok());
}

/// @covers: NoopGrpcValidator::run_validate — happy path
#[test]
fn test_run_validate_noop_always_ok_happy() {
    let v = NoopGrpcValidator::create();
    assert!(v.run_validate().is_ok());
}

/// @covers: NoopGrpcValidator::run_validate — no error path (documents absence)
#[test]
fn test_run_validate_noop_no_error_path_error() {
    let v = NoopGrpcValidator::create();
    assert!(v.run_validate().is_ok());
}

/// @covers: NoopGrpcValidator::run_validate — called twice is idempotent
#[test]
fn test_run_validate_noop_called_twice_edge() {
    let v = NoopGrpcValidator::create();
    assert!(v.run_validate().is_ok());
    assert!(v.run_validate().is_ok());
}
