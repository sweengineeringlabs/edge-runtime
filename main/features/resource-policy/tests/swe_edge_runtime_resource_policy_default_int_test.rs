//! Integration tests for api/swe/default — default implementation contract.

use swe_edge_runtime_resource_policy::{
    SweEdgeRuntimeResourcePolicy, SweEdgeRuntimeResourcePolicyFactory,
};

/// @covers: api::swe::default::swe_edge_runtime_resource_policy
#[test]
fn test_default_swe_edge_runtime_resource_policy_executes_via_factory() {
    let svc = SweEdgeRuntimeResourcePolicyFactory::create_swe_edge_runtime_resource_policy();
    assert!(svc.execute().is_ok());
}

/// @covers: api::swe::default::swe_edge_runtime_resource_policy
#[test]
fn test_default_swe_edge_runtime_resource_policy_is_object_safe() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<&dyn SweEdgeRuntimeResourcePolicy>();
}
