//! SAF layer — public factory surface for resource policy.

mod policy_svc;

pub use crate::api::{
    Error, NoopRuntimeResourcePolicy, PolicySvc, ResourceLimits, ResourceLimitsBuilder,
    ResourceLimitsResolver, ResourcePolicy, ResourcePolicyBuilder, ResourcePolicyConfig,
    ResourcePolicyError, ResourcePolicyRunner, SweEdgeRuntimeResourcePolicy,
    SweEdgeRuntimeResourcePolicyFactory, Validator,
};
