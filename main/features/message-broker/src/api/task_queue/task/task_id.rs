//! [`TaskId`] — unique identifier for a task.

use uuid::Uuid;

/// Unique identifier for a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(uuid::Uuid);

impl TaskId {
    /// Generate a new random task ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a TaskId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_task_id_new_generates_uuid() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_task_id_default_generates_uuid() {
        let id1 = TaskId::default();
        let id2 = TaskId::default();
        assert_ne!(id1, id2);
    }

    /// @covers: from_uuid
    #[test]
    fn test_task_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = TaskId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), uuid);
    }

    /// @covers: as_uuid
    #[test]
    fn test_task_id_as_uuid_returns_underlying_uuid() {
        let uuid = Uuid::nil();
        let id = TaskId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), uuid);
    }

    #[test]
    fn test_task_id_display() {
        let uuid = Uuid::nil();
        let id = TaskId::from_uuid(uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }
}
