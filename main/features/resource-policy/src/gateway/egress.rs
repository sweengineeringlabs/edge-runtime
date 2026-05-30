//! Gateway egress — public surface re-exports.

pub use crate::api::error::{Error, ResourcePolicyError};
pub use crate::api::traits::{SweEdgeRuntimeResourcePolicy, Validator};
pub use crate::api::types::policy::{
    ResourceLimits, ResourceLimitsBuilder, ResourceLimitsResolver, ResourcePolicy,
    ResourcePolicyBuilder, ResourcePolicyConfig, ResourcePolicyRunner,
};
pub use crate::api::types::swe::{PolicySvc, SweEdgeRuntimeResourcePolicyFactory};
