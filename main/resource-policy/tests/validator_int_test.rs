//! Integration tests for the Validator trait.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, Validator};

/// @covers: Validator
#[test]
fn test_validator_validates_successfully() {
    let v = SweEdgeRuntimeResourcePolicyFactory::create_validator();
    assert!(v.validate().is_ok());
}
