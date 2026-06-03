//! Integration tests for [`NoopRuntimeResourcePolicy`].

use swe_edge_runtime_resource_policy::{
    NoopRuntimeResourcePolicy, SweEdgeRuntimeResourcePolicy, Validator,
};

/// @covers: NoopRuntimeResourcePolicy::execute
#[test]
fn test_execute_returns_ok() {
    let policy = NoopRuntimeResourcePolicy;
    assert!(policy.execute().is_ok());
}

/// @covers: NoopRuntimeResourcePolicy::validate
#[test]
fn test_validate_returns_ok() {
    let policy = NoopRuntimeResourcePolicy;
    assert!(policy.validate().is_ok());
}

/// @covers: NoopRuntimeResourcePolicy — default construction
#[test]
fn test_default_creates_noop_policy() {
    let _policy = NoopRuntimeResourcePolicy;
}
