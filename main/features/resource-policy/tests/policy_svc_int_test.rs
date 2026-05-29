//! Integration tests for swe-edge-runtime-resource-policy SAF facade.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, SweEdgeRuntimeResourcePolicy, Validator};

/// @covers: SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy
#[test]
fn test_create_swe_edge_runtime_resource_policy_via_saf_succeeds() {
    let svc = SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}

/// @covers: SweEdgeRuntimeResourcePolicyFactory::create_validator
#[test]
fn test_create_validator_via_saf_succeeds() {
    let v = SweEdgeRuntimeResourcePolicyFactory::create_validator();
    assert!(v.validate().is_ok());
}
