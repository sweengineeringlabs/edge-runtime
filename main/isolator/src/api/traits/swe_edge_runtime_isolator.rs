//! `SweEdgeRuntimeIsolator` — primary service trait for swe_edge_runtime_isolator.

use crate::api::error::Error;

/// Primary service trait for swe_edge_runtime_isolator.
pub trait SweEdgeRuntimeIsolator: Send + Sync {
    /// Execute the primary operation.
    fn execute(&self) -> Result<(), Error>;
}
