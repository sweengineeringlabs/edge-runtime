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
    assert!(v.run_validate().is_ok());
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
    assert!(a.run_validate().is_ok());
    assert!(b.run_validate().is_ok());
}

// ─── run_validate ────────────────────────────────────────────────────────────

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_returns_ok_happy() {
    let v = NoopValidator::create();
    assert!(v.run_validate().is_ok());
}

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_never_errors_error() {
    let v = NoopValidator::create();
    let result = v.run_validate();
    assert!(
        result.is_ok(),
        "NoopValidator::run_validate must never return an error"
    );
}

/// @covers: NoopValidator::run_validate
#[test]
fn test_run_validate_idempotent_edge() {
    let v = NoopValidator::create();
    assert!(v.run_validate().is_ok());
    assert!(v.run_validate().is_ok());
}
