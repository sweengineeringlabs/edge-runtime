//! API interface surface for the in-memory task queue implementation.

#[allow(clippy::module_inception)]
pub mod in_memory_task_queue;

pub use in_memory_task_queue::InMemoryTaskQueue;
