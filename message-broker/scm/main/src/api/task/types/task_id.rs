//! [`TaskId`] — unique identifier for a task.

/// Unique identifier for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub(crate) uuid::Uuid);
