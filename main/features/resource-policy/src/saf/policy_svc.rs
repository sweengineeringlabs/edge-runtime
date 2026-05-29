//! Factory functions for SweEdgeRuntimeResourcePolicy and Validator.

use crate::api::swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicy;
use crate::api::traits::Validator;

/// Create a default SweEdgeRuntimeResourcePolicy implementation.
pub fn create_swe_edge_runtime_resource_policy() -> impl SweEdgeRuntimeResourcePolicy {
    crate::core::DefaultSweEdgeRuntimeResourcePolicy::new()
}

/// Create a default Validator implementation.
pub fn create_validator() -> impl Validator {
    crate::core::DefaultSweEdgeRuntimeResourcePolicy::new()
}
