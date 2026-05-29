//! API interface surface for the NATS task queue implementation.

#[allow(clippy::module_inception)]
pub mod nats_task_queue;

pub use nats_task_queue::NatsTaskQueue;
