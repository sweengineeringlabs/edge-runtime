//! Integration tests for the `validator_svc` SAF surface.
// @covers NoopValidator::create
// @covers NoopValidator::run_validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::NoopValidator;

// ─── create ─────────────────────────────────────────────────────────────────

/// @covers: NoopValidator::create
#[test]
fn test_create_returns_usable_validator_happy() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    result.unwrap();
}

/// @covers: NoopValidator::create
#[test]
fn test_create_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<NoopValidator>(), 0);
}

/// @covers: NoopValidator::create
#[test]
fn test_create_independent_instances_edge() {
    let a = NoopValidator::create();
    let b = NoopValidator::create();
    let a_result = a.run_validate();
    let b_result = b.run_validate();
    assert!(a_result.is_ok());
    assert!(b_result.is_ok());
    a_result.unwrap();
    b_result.unwrap();
}

// ─── run_validate ────────────────────────────────────────────────────────────

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_returns_ok_happy() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    result.unwrap();
}

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_never_errors_error() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(result.is_ok());
    result.unwrap();
}

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_idempotent_edge() {
    let v = NoopValidator::create();
    let first = v.run_validate();
    let second = v.run_validate();
    assert!(first.is_ok());
    assert!(second.is_ok());
    first.unwrap();
    second.unwrap();
}
