//! Integration tests for [`NoopValidator`].
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{NoopValidator, Validator};

/// @covers: NoopValidator::validate
#[test]
fn test_validate_returns_ok_happy() {
    let v = NoopValidator::create();
    assert!(v.validate().is_ok());
}

/// @covers: NoopValidator::validate
#[test]
fn test_validate_ok_documents_absent_error_path_error() {
    // NoopValidator never errors; verify the Result is Ok (documents the error path is absent).
    let v = NoopValidator::create();
    let result = v.validate();
    assert!(result.is_ok(), "NoopValidator must never return an error");
}

/// @covers: NoopValidator::validate
#[test]
fn test_validate_idempotent_edge() {
    let v = NoopValidator::create();
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}
