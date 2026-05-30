//! SAF layer — public factory surface for resource policy.

mod policy_svc;

pub use crate::api::{
    Error, PolicySvc, ResourceLimits, ResourceLimitsBuilder, ResourceLimitsResolver,
    ResourcePolicy, ResourcePolicyBuilder, ResourcePolicyConfig, ResourcePolicyError,
    ResourcePolicyRunner, SweEdgeRuntimeResourcePolicy, SweEdgeRuntimeResourcePolicyFactory,
    Validator,
};
