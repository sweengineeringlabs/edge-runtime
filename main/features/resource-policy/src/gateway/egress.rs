//! Gateway egress — public surface re-exports.

pub use crate::api::error::{Error, ResourcePolicyError};
pub use crate::api::traits::{SweEdgeRuntimeResourcePolicy, Validator};
pub use crate::api::types::swe::{
    SweEdgeRuntimeResourcePolicyFactory,
    DefaultSweEdgeRuntimeResourcePolicyImpl,
    DefaultSweEdgeRuntimeResourcePolicyValidatorImpl,
    PolicySvc,
};
pub use crate::api::types::policy::{
    ResourceLimits,
    ResourceLimitsResolver,
    ResourcePolicy,
    ResourcePolicyConfig,
    ResourcePolicyRunner,
};
