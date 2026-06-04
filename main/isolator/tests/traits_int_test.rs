//! Integration tests for swe-edge-runtime-isolator API traits.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, Validator};

/// @covers: Validator
#[test]
fn test_validator_trait_is_object_safe() {
    fn _accept(_v: &dyn Validator) {}
}

/// @covers: SweEdgeRuntimeIsolatorFactory::create_validator
#[test]
fn test_create_validator_returns_valid_impl() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}
