//! `swe_edge_message_broker` — cross-process pub/sub broker.
//!
//! Provides a runtime-agnostic [`MessageBroker`] trait for cross-process
//! publish/subscribe messaging.  Use [`in_memory_broker`] for testing and
//! local services, [`nats_broker`] for NATS-backed production deployments.

mod api;
mod core;
mod saf;
mod spi;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
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
pub use crate::saf::in_memory_broker;
#[cfg(feature = "nats")]
pub use crate::saf::nats_broker;

#[cfg(feature = "tokio-rt")]
pub use crate::saf::in_memory_task_queue;
#[cfg(feature = "nats")]
pub use crate::saf::nats_task_queue;
