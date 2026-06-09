//! `RuntimeResult` — shorthand for fallible runtime operations.

use crate::api::runtime::errors::runtime::runtime_error::RuntimeError;

/// Result type for runtime manager operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;
