//! Task queue value types — structs used by the TaskQueue API.

#[cfg(feature = "tokio-rt")]
pub mod in_memory_task_queue;
pub mod task;
