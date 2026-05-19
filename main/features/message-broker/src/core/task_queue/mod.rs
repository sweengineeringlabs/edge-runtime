//! Task queue implementations.

#[cfg(feature = "tokio-rt")]
pub(crate) mod in_memory_task_queue;
#[cfg(feature = "nats")]
pub(crate) mod nats_task_queue;
// kafka feature is placeholder for now
