//! Gateway egress integration tests.

use swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicyFactory;

/// @covers: egress
#[test]
fn test_factory_accessible_through_crate_root() {
    let _ = std::mem::size_of::<SweEdgeRuntimeResourcePolicyFactory>();
}
