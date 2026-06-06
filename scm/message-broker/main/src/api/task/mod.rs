//! Task theme — competing-consumer task queue port, value types, and errors.

pub(crate) mod error;
pub(crate) mod traits;
pub(crate) mod types;

pub use error::QueueError;
pub use traits::TaskQueue;
pub use types::{Task, TaskHandle, TaskId, TaskQueueFactory};
