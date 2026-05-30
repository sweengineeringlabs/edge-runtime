//! SAF layer — public factory surface for resource policy.

mod policy_svc;

pub use crate::api::{
    DefaultSweEdgeRuntimeResourcePolicyImpl, DefaultSweEdgeRuntimeResourcePolicyValidatorImpl,
    Error, PolicySvc, ResourceLimits, ResourceLimitsResolver, ResourcePolicy, ResourcePolicyConfig,
    ResourcePolicyError, ResourcePolicyRunner, SweEdgeRuntimeResourcePolicy,
    SweEdgeRuntimeResourcePolicyFactory, Validator,
};
