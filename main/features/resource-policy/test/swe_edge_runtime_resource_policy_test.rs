//! Unit tests for swe-edge-runtime-resource-policy.

use swe_edge_runtime_resource_policy::*;

/// @covers: create_swe_edge_runtime_resource_policy
#[test]
fn test_create_swe_edge_runtime_resource_policy_returns_working_impl() {
    let svc = create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}
