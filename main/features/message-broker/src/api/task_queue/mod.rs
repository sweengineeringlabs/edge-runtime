//! Task queue API — traits, value types, and error definitions.

pub mod queue_error;
pub mod task;
pub mod task_handle;
pub mod task_queue;

pub use queue_error::QueueError;
pub use task::{Task, TaskId};
pub use task_handle::TaskHandle;
pub use task_queue::TaskQueue;
