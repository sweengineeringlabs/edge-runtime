//! `SweEdgeRuntimeResourcePolicy` — primary service trait.

use crate::api::error::error::Error;

/// Primary service trait for swe_edge_runtime_resource_policy.
pub trait SweEdgeRuntimeResourcePolicy: Send + Sync {
    /// Execute the primary operation.
    fn execute(&self) -> Result<(), Error>;
}
