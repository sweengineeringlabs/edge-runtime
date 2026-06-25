//! Integration tests for [`NoopValidator`].
// @covers NoopValidator::validate
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{NoopValidator, Validator};

/// @covers: NoopValidator::validate
#[test]
fn test_validate_returns_ok_happy() {
    let v = NoopValidator::create();
    assert_eq!(v.validate().ok(), Some(()));
}

/// @covers: NoopValidator::validate
#[test]
fn test_validate_ok_documents_absent_error_path_error() {
    // NoopValidator never errors; verify the Result carries Ok(()) (documents the error path is absent).
    let v = NoopValidator::create();
    let result = v.validate();
    assert_eq!(result.ok(), Some(()));
}

/// @covers: NoopValidator::validate
#[test]
fn test_validate_idempotent_edge() {
    let v = NoopValidator::create();
    assert_eq!(v.validate().ok(), Some(()));
    assert_eq!(v.validate().ok(), Some(()));
}
