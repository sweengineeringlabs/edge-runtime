//! SAF layer — message broker and task queue public facade.
//!
//! Single entry point: broker_svc.

mod broker_svc;

#[cfg(feature = "tokio-rt")]
pub use crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;
pub use crate::api::broker::BrokerError;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageBrokerFactory;
pub use crate::api::broker::MessageStream;
#[cfg(feature = "tokio-rt")]
pub use crate::api::task::types::in_memory_task_queue::InMemoryTaskQueue;
pub use crate::api::task::QueueError;
pub use crate::api::task::Task;
pub use crate::api::task::TaskHandle;
pub use crate::api::task::TaskId;
pub use crate::api::task::TaskQueue;
pub use crate::api::task::TaskQueueFactory;
