//! Integration tests for swe-edge-runtime-resource-policy API traits.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, Validator};

/// @covers: Validator
#[test]
fn test_validator_trait_is_object_safe() {
    fn _accept(_v: &dyn Validator) {}
}

/// @covers: SweEdgeRuntimeResourcePolicyFactory::create_validator
#[test]
fn test_create_validator_returns_valid_impl() {
    let v = SweEdgeRuntimeResourcePolicyFactory::create_validator();
    assert!(v.validate().is_ok());
}
