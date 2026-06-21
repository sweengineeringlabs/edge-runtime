//! [`TaskQueueFactoryContract`] — contract for types that construct task queue instances.

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task_handle_builder::TaskHandleBuilder;
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

    /// Return a [`TaskHandleBuilder`] pre-seeded with the dequeued task's identity.
    ///
    /// Convenience factory so implementors can construct [`TaskHandle`] values
    /// without depending on the concrete builder type.
    fn build_handle(
        task_id: TaskId,
        payload: Bytes,
        headers: HashMap<String, String>,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> TaskHandleBuilder {
        TaskHandleBuilder::new(task_id, payload, ack, nack).headers(headers)
    }
}

impl TaskQueueFactoryContract for TaskQueueFactory {
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(&self) -> InMemoryTaskQueue {
        TaskQueueFactory::in_memory()
    }
}
