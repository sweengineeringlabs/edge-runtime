//! SweEdgeRuntimeResourcePolicy trait definition.
//!
//! Implement this trait in core/ to define swe_edge_runtime_resource_policy's primary behavior.

use super::error::Error;

/// Primary service trait for swe_edge_runtime_resource_policy.
pub trait SweEdgeRuntimeResourcePolicy: Send + Sync {
    /// Execute the primary operation.
    fn execute(&self) -> Result<(), Error>;
}
