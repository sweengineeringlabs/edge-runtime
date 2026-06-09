//! Task theme — competing-consumer task queue port, value types, and errors.

pub(crate) mod errors;
pub(crate) mod queue;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::QueueError;
pub use traits::TaskQueue;
pub use types::{Task, TaskHandle, TaskId, TaskQueueFactory};
