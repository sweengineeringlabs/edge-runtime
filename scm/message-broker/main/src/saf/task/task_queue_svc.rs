//! SAF — [`TaskQueue`] public service surface.
//!
//! Exposes the [`TaskQueue`] trait and related types for consumers that need to
//! enqueue and dequeue tasks using a competing-consumer work queue.

pub use crate::api::task::traits::TaskQueue;

/// Maximum task payload size in bytes accepted by the default in-memory queue.
pub const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
pub use crate::api::task::errors::queue_error::QueueError;
pub use crate::api::task::types::task::Task;
pub use crate::api::task::types::task_handle::TaskHandle;
pub use crate::api::task::types::task_id::TaskId;
pub use crate::api::task::types::task_queue_factory::TaskQueueFactory;

#[cfg(feature = "tokio-rt")]
pub use crate::api::task::types::in_memory_task_queue::InMemoryTaskQueue;
