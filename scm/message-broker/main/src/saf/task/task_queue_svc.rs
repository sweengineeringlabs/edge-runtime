//! SAF — [`TaskQueue`] public service surface.
//!
//! Exposes the [`TaskQueue`] trait and related types for consumers that need to
//! enqueue and dequeue tasks using a competing-consumer work queue.

pub use crate::api::TaskQueue;

/// Maximum task payload size in bytes accepted by the default in-memory queue.
pub const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
pub use crate::api::QueueError;
pub use crate::api::Task;
pub use crate::api::TaskHandle;
pub use crate::api::TaskHandleBuilder;
pub use crate::api::TaskId;
pub use crate::api::TaskQueueFactory;

#[cfg(feature = "tokio-rt")]
pub use crate::api::InMemoryTaskQueue;
