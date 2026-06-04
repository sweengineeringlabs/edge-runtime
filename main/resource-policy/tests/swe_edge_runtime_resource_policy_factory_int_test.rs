//! Integration tests for SweEdgeRuntimeResourcePolicyFactory.

use swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicyFactory;

/// @covers: SweEdgeRuntimeResourcePolicyFactory
#[test]
fn test_swe_edge_runtime_resource_policy_factory_size_is_known() {
    let _ = std::mem::size_of::<SweEdgeRuntimeResourcePolicyFactory>();
}
