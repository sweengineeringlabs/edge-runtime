//! In-memory task queue — [`InMemoryTaskQueue`] backed by tokio mpsc.

#[allow(clippy::module_inception)]
mod in_memory_task_queue;

pub(crate) use in_memory_task_queue::InMemoryTaskQueue;
