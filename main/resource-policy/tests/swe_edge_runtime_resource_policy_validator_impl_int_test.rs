//! Integration tests for DefaultSweEdgeRuntimeResourcePolicyValidatorImpl.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, Validator};

/// @covers: DefaultSweEdgeRuntimeResourcePolicyValidatorImpl
#[test]
fn test_default_validator_impl_validates() {
    let v = SweEdgeRuntimeResourcePolicyFactory::create_validator();
    assert!(v.validate().is_ok());
}
