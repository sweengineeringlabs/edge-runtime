//! Gateway egress — public surface re-exports.

pub use crate::api::error::Error;
pub use crate::api::traits::{SweEdgeRuntimeIsolator, Validator};
pub use crate::api::types::swe::{
    SweEdgeRuntimeIsolatorFactory,
    DefaultSweEdgeRuntimeIsolatorImpl,
    DefaultSweEdgeRuntimeIsolatorValidatorImpl,
    IsolatorSvc,
};
pub use crate::api::types::profile::{IsolationProfileRegistry, IsolatorConfig, ProfileSpec};
