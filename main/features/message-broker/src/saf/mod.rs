//! SAF layer — message broker and task queue public facade.
//!
//! Single entry point: edge_message_broker_svc and edge_task_queue_svc.

mod edge_message_broker_svc;
mod edge_task_queue_svc;

pub use crate::api::broker::BrokerError;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::api::task_queue::QueueError;
pub use crate::api::task_queue::Task;
pub use crate::api::task_queue::TaskHandle;
pub use crate::api::task_queue::TaskId;
pub use crate::api::task_queue::TaskQueue;
pub use crate::api::traits::Validator;

#[cfg(feature = "tokio-rt")]
pub use edge_message_broker_svc::in_memory_broker;
#[cfg(feature = "nats")]
pub use edge_message_broker_svc::nats_broker;

#[cfg(feature = "nats")]
pub use edge_task_queue_svc::nats_task_queue;
