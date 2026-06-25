//! Integration tests for `saf::validator_svc` — the SAF validator surface.
// @covers NoopValidator::create
// @covers NoopValidator::run_validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::NoopValidator;

#[test]
fn test_create_noop_validator_run_validate_happy() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (), "run_validate must return Ok(())");
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
    assert_eq!(
        result.unwrap(),
        (),
        "NoopValidator::run_validate must return unit"
    );
}

#[test]
fn test_create_noop_validator_idempotent_edge() {
    // Edge: multiple create() calls each produce independent validators.
    let v1 = NoopValidator::create();
    let v2 = NoopValidator::create();
    let r1 = v1.run_validate();
    let r2 = v2.run_validate();
    assert!(r1.is_ok());
    assert_eq!(
        r1.unwrap(),
        (),
        "first instance run_validate must return Ok(())"
    );
    assert!(r2.is_ok());
    assert_eq!(
        r2.unwrap(),
        (),
        "second instance run_validate must return Ok(())"
    );
}

#[test]
fn test_run_validate_noop_always_ok_happy() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (), "run_validate must return Ok(())");
}

#[test]
fn test_run_validate_noop_no_error_path_error() {
    // Documents that no error path exists for NoopValidator::run_validate.
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        (),
        "no error path — unit value must be returned"
    );
}

#[test]
fn test_run_validate_noop_called_twice_edge() {
    // Edge: calling run_validate twice is idempotent.
    let v = NoopValidator::create();
    let r1 = v.run_validate();
    let r2 = v.run_validate();
    assert!(r1.is_ok());
    assert_eq!(r1.unwrap(), (), "first run_validate must return Ok(())");
    assert!(r2.is_ok());
    assert_eq!(
        r2.unwrap(),
        (),
        "second run_validate must return Ok(()) (idempotent)"
    );
}
