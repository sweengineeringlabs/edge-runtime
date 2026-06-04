//! SAF layer — public factory surface for resource policy.

mod policy_svc;

#[expect(
    unused_imports,
    reason = "SEA saf/ anchor — all items exported for consumers, not used internally"
)]
pub use crate::api::{
    Error, NoopRuntimeResourcePolicy, PolicySvc, ResourceLimits, ResourceLimitsBuilder,
    ResourceLimitsResolver, ResourcePolicy, ResourcePolicyBuilder, ResourcePolicyConfig,
    ResourcePolicyError, ResourcePolicyRunner, SweEdgeRuntimeResourcePolicy,
    SweEdgeRuntimeResourcePolicyFactory, Validator,
};
