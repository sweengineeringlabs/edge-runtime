//! SAF layer — message broker and task queue public facade.
//!
//! Single entry point: broker_svc.

mod broker_svc;

pub use crate::api::broker::BrokerError;
pub use crate::api::broker::Message;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::api::task_queue::QueueError;
pub use crate::api::task_queue::Task;
pub use crate::api::task_queue::TaskHandle;
pub use crate::api::task_queue::TaskId;
pub use crate::api::task_queue::TaskQueue;
pub use crate::api::types::MessageBrokerFactory;
pub use crate::api::types::TaskQueueFactory;
