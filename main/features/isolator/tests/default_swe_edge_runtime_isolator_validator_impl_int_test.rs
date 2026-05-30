//! Integration tests for the default Validator implementation.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, Validator};

/// @covers: SweEdgeRuntimeIsolatorFactory::create_validator
#[test]
fn test_default_validator_impl_is_accessible_via_factory() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}

/// @covers: SweEdgeRuntimeIsolatorFactory::create_validator
#[test]
fn test_default_validator_impl_validate_is_idempotent() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok(), "first validate must succeed");
    assert!(
        v.validate().is_ok(),
        "second validate must succeed (idempotent)"
    );
}
