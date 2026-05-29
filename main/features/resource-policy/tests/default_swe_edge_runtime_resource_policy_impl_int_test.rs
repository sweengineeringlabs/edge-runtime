//! Integration tests for DefaultSweEdgeRuntimeResourcePolicyImpl.

use swe_edge_runtime_resource_policy::{SweEdgeRuntimeResourcePolicyFactory, SweEdgeRuntimeResourcePolicy};

/// @covers: DefaultSweEdgeRuntimeResourcePolicyImpl
#[test]
fn test_default_impl_executes_via_factory() {
    let svc = SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}

/// @covers: DefaultSweEdgeRuntimeResourcePolicyImpl
#[test]
fn test_default_impl_type_size_is_known() {
    use swe_edge_runtime_resource_policy::DefaultSweEdgeRuntimeResourcePolicyImpl;
    let _ = std::mem::size_of::<DefaultSweEdgeRuntimeResourcePolicyImpl>();
}
