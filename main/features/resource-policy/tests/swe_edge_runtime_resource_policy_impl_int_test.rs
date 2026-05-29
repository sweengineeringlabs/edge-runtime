//! Integration tests for DefaultSweEdgeRuntimeResourcePolicyImpl.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, SweEdgeRuntimeResourcePolicy};

/// @covers: DefaultSweEdgeRuntimeResourcePolicyImpl
#[test]
fn test_default_impl_executes() {
    let svc = SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}
