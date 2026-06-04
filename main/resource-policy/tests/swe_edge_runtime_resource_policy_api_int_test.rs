//! API trait integration tests for swe-edge-runtime-resource-policy.

use swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicy;

/// @covers: SweEdgeRuntimeResourcePolicy
#[test]
fn test_swe_edge_runtime_resource_policy_trait_is_object_safe() {
    fn _accept(_s: &dyn SweEdgeRuntimeResourcePolicy) {}
}
