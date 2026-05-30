//! SAF layer — message broker and task queue public facade.
//!
//! Single entry point: broker_svc.

mod broker_svc;

#[cfg(feature = "tokio-rt")]
pub use crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;
pub use crate::api::broker::BrokerError;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
#[cfg(feature = "tokio-rt")]
pub use crate::api::task::queue::types::in_memory_task_queue::InMemoryTaskQueue;
pub use crate::api::task::queue::QueueError;
pub use crate::api::task::queue::Task;
pub use crate::api::task::queue::TaskHandle;
pub use crate::api::task::queue::TaskId;
pub use crate::api::task::queue::TaskQueue;
pub use crate::api::types::MessageBrokerFactory;
pub use crate::api::types::TaskQueueFactory;
