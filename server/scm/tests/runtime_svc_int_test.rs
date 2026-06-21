//! Integration tests for the runtime_svc SAF surface.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime::RUNTIME_API_VERSION;

/// @covers: RUNTIME_API_VERSION
#[test]
fn test_runtime_api_version_const_is_non_empty_happy() {
    assert!(!RUNTIME_API_VERSION.is_empty());
}

#[test]
fn test_runtime_api_version_contains_no_whitespace_error() {
    assert!(!RUNTIME_API_VERSION.contains(char::is_whitespace));
}

#[test]
fn test_runtime_api_version_starts_with_digit_edge() {
    let first = RUNTIME_API_VERSION.chars().next().unwrap();
    assert!(
        first.is_ascii_digit(),
        "version must start with a digit: {RUNTIME_API_VERSION}"
    );
}
