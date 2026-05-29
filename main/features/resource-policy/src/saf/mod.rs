//! SAF layer — public factory surface for resource policy.

mod policy_svc;

pub use crate::api::{
    Error,
    ResourceLimits,
    ResourceLimitsResolver,
    ResourcePolicy,
    ResourcePolicyConfig,
    ResourcePolicyError,
    ResourcePolicyRunner,
    SweEdgeRuntimeResourcePolicy,
    SweEdgeRuntimeResourcePolicyFactory,
    DefaultSweEdgeRuntimeResourcePolicyImpl,
    DefaultSweEdgeRuntimeResourcePolicyValidatorImpl,
    Validator,
    PolicySvc,
};
