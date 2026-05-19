//! SAF layer — message broker and task queue public facade.

mod edge_message_broker_svc;
mod edge_task_queue_svc;

#[cfg(feature = "tokio-rt")]
pub use edge_message_broker_svc::in_memory_broker;
#[cfg(feature = "nats")]
pub use edge_message_broker_svc::nats_broker;

#[cfg(feature = "tokio-rt")]
pub use edge_task_queue_svc::in_memory_task_queue;
#[cfg(feature = "nats")]
pub use edge_task_queue_svc::nats_task_queue;
