//! Integration tests for swe-edge-runtime-resource-policy API traits.

use swe_edge_runtime_resource_policy::*;

/// @covers: Validator
#[test]
fn test_validator_trait_is_object_safe() {
    fn _accept(_v: &dyn Validator) {}
}

/// @covers: create_validator
#[test]
fn test_create_validator_returns_valid_impl() {
    let v = create_validator();
    assert!(v.validate().is_ok());
}
