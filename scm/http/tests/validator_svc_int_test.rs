//! Integration tests for `saf::validator_svc` — the SAF validator surface.
// @covers NoopValidator::create
// @covers NoopValidator::run_validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::NoopValidator;

#[test]
fn test_create_noop_validator_run_validate_happy() {
    let v = NoopValidator::create();
    assert!(v.run_validate().is_ok());
}

#[test]
fn test_create_noop_validator_never_errors_error() {
    // create() + run_validate() never returns Err.
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(
        result.is_ok(),
        "NoopValidator::run_validate must never fail"
    );
}

#[test]
fn test_create_noop_validator_idempotent_edge() {
    // Edge: multiple create() calls each produce independent validators.
    let v1 = NoopValidator::create();
    let v2 = NoopValidator::create();
    assert!(v1.run_validate().is_ok());
    assert!(v2.run_validate().is_ok());
}

#[test]
fn test_run_validate_noop_always_ok_happy() {
    let v = NoopValidator::create();
    assert!(v.run_validate().is_ok());
}

#[test]
fn test_run_validate_noop_no_error_path_error() {
    // Documents that no error path exists for NoopValidator::run_validate.
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
}

#[test]
fn test_run_validate_noop_called_twice_edge() {
    // Edge: calling run_validate twice is idempotent.
    let v = NoopValidator::create();
    assert!(v.run_validate().is_ok());
    assert!(v.run_validate().is_ok());
}
