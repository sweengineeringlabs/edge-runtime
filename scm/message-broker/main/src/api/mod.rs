//! API layer — public trait contracts and value types.

mod broker;
mod config;
mod task;

// Broker theme — flat re-exports
pub use broker::traits::broker_factory::BrokerFactory;
#[cfg(feature = "tokio-rt")]
pub use broker::types::in_memory_message_broker::InMemoryMessageBroker;
pub use broker::BrokerError;
pub use broker::Message;
pub use broker::MessageBroker;
pub use broker::MessageBrokerFactory;
pub use broker::MessageStream;

// Config theme — flat re-exports
pub use config::traits::config_provider::ConfigProvider;
pub use config::traits::validator::Validator;
pub use config::types::application_config::ApplicationConfig;
pub use config::types::broker_backend_config::BrokerBackendConfig;

// Task theme — flat re-exports
pub use task::errors::queue_error::QueueError;
#[cfg(feature = "tokio-rt")]
pub use task::queue::MAX_QUEUE_DEPTH;
pub use task::traits::task_queue::TaskQueue;
pub use task::traits::task_queue_factory_contract::TaskQueueFactoryContract;
#[cfg(feature = "tokio-rt")]
pub use task::types::in_memory_task_queue::InMemoryTaskQueue;
pub use task::types::task::Task;
pub use task::types::task_handle::TaskHandle;
pub use task::types::task_handle_builder::TaskHandleBuilder;
pub use task::types::task_id::TaskId;
pub use task::types::task_queue_factory::TaskQueueFactory;
