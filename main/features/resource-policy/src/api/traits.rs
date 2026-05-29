//! Canonical trait re-exports for swe_edge_runtime_resource_policy.
//!
//! Reviewers check this file for the primary service trait and Validator.
pub use super::swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicy;

use super::error::Error;

/// Validates swe_edge_runtime_resource_policy inputs before execution.
pub trait Validator: Send + Sync {
    /// Validate inputs, returning an error if invalid.
    fn validate(&self) -> Result<(), Error>;
}
