//! Integration tests for DefaultSweEdgeRuntimeResourcePolicyValidatorImpl.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, Validator};

/// @covers: DefaultSweEdgeRuntimeResourcePolicyValidatorImpl
#[test]
fn test_default_validator_impl_validates_via_factory() {
    let v = SweEdgeRuntimeResourcePolicyFactory::create_validator();
    assert!(v.validate().is_ok());
}

/// @covers: DefaultSweEdgeRuntimeResourcePolicyValidatorImpl
#[test]
fn test_default_validator_impl_type_size_is_known() {
    use swe_edge_runtime_resource_policy::DefaultSweEdgeRuntimeResourcePolicyValidatorImpl;
    let _ = std::mem::size_of::<DefaultSweEdgeRuntimeResourcePolicyValidatorImpl>();
}
