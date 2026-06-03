//! Integration tests for [`NoopRuntimeIsolator`].

use swe_edge_runtime_isolator::{NoopRuntimeIsolator, SweEdgeRuntimeIsolator, Validator};

/// @covers: NoopRuntimeIsolator::execute
#[test]
fn test_execute_returns_ok() {
    let isolator = NoopRuntimeIsolator;
    assert!(isolator.execute().is_ok());
}

/// @covers: NoopRuntimeIsolator::validate
#[test]
fn test_validate_returns_ok() {
    let isolator = NoopRuntimeIsolator;
    assert!(isolator.validate().is_ok());
}

/// @covers: NoopRuntimeIsolator — default construction
#[test]
fn test_default_creates_noop_isolator() {
    let _isolator = NoopRuntimeIsolator;
}
