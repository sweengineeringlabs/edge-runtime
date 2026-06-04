//! DefaultSweEdgeRuntimeResourcePolicy integration tests for swe-edge-runtime-resource-policy.

use swe_edge_runtime_resource_policy::{
    SweEdgeRuntimeResourcePolicy, SweEdgeRuntimeResourcePolicyFactory,
};

/// @covers: SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy
#[test]
fn test_default_swe_edge_runtime_resource_policy_creates_and_executes() {
    let svc = SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}
