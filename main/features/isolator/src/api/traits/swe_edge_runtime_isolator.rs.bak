//! SweEdgeRuntimeIsolator trait definition.
//!
//! Implement this trait in core/ to define swe_edge_runtime_isolator's primary behavior.

use super::error::Error;

/// Primary service trait for swe_edge_runtime_isolator.
pub trait SweEdgeRuntimeIsolator: Send + Sync {
    /// Execute the primary operation.
    fn execute(&self) -> Result<(), Error>;
}
