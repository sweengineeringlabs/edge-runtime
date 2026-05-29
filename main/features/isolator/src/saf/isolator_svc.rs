//! Factory functions for SweEdgeRuntimeIsolator and Validator.

use crate::api::swe_edge_runtime_isolator::SweEdgeRuntimeIsolator;
use crate::api::traits::Validator;

/// Create a default SweEdgeRuntimeIsolator implementation.
pub fn create_swe_edge_runtime_isolator() -> impl SweEdgeRuntimeIsolator {
    crate::core::DefaultSweEdgeRuntimeIsolator::new()
}

/// Create a default Validator implementation.
pub fn create_validator() -> impl Validator {
    crate::core::DefaultSweEdgeRuntimeIsolator::new()
}
