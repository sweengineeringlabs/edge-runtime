//! [`SchedulerResult`] — scheduler operation result type.

use crate::api::error::SchedulerError;

/// Result type for scheduler operations.
pub type SchedulerResult<T> = Result<T, SchedulerError>;
