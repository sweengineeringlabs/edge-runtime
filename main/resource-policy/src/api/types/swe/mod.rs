//! SweEdgeRuntimeResourcePolicy factory and implementation types.

pub mod noop_runtime_resource_policy;
pub mod policy_svc;
pub mod swe_edge_runtime_resource_policy_factory;

pub use noop_runtime_resource_policy::NoopRuntimeResourcePolicy;
pub use policy_svc::PolicySvc;
pub use swe_edge_runtime_resource_policy_factory::SweEdgeRuntimeResourcePolicyFactory;
