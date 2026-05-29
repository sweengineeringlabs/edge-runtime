//! Validator trait.

use crate::api::error::Error;

/// Validates swe_edge_runtime_resource_policy inputs before execution.
pub trait Validator: Send + Sync {
    /// Validate inputs, returning an error if invalid.
    fn validate(&self) -> Result<(), Error>;
}
