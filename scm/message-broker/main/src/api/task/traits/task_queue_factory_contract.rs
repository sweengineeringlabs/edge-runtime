//! [`TaskQueueFactoryContract`] — contract for types that construct task queue instances.

use crate::api::task::types::task_id::TaskId;
use crate::api::task::types::task_queue_factory::TaskQueueFactory;

#[cfg(feature = "tokio-rt")]
use crate::api::task::types::in_memory_task_queue::InMemoryTaskQueue;

/// Contract for types that construct task queue instances.
///
/// Implementors produce concrete task queue instances and expose the factory type
/// that consumers use at construction time. [`TaskQueueFactory`] is the canonical
/// implementor in this crate.
pub trait TaskQueueFactoryContract {
    /// Return the default factory instance for constructing task queues.
    fn default_factory() -> TaskQueueFactory {
        TaskQueueFactory
    }

    /// Generate a fresh [`TaskId`] for use when constructing tasks.
    fn new_task_id(&self) -> TaskId {
        TaskId::new()
    }

    /// Construct an in-memory task queue backed by [`tokio::sync::mpsc`].
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(&self) -> InMemoryTaskQueue;
}

impl TaskQueueFactoryContract for TaskQueueFactory {
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(&self) -> InMemoryTaskQueue {
        TaskQueueFactory::in_memory()
    }
}
