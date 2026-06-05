//! Task queue API — traits, value types, and error definitions.

#[cfg(feature = "kafka")]
pub mod kafka;
pub mod memory;
pub mod nats;
pub mod queue_error;
pub mod task;
pub mod task_queue;
pub mod types;

pub use queue_error::QueueError;
pub use task::{Task, TaskId};
pub use task_queue::TaskQueue;
pub use types::task::TaskHandle;
