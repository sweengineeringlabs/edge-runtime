//! SAF layer — message broker and task queue public facade.
//!
//! All public items flow through `_svc.rs` files; this module only
//! declares submodules and re-exports from them.

pub mod broker;
mod config_provider_svc;
pub mod task;
mod validator_svc;

pub use broker::BrokerErr as BrokerError;
pub use broker::BrokerFactory;
pub use broker::BrokerMessage as Message;
pub use broker::MessageBrokerFactory;

pub use config_provider_svc::ApplicationConfig;
pub use config_provider_svc::BrokerBackendConfig;
pub use config_provider_svc::ConfigProvider;
pub use config_provider_svc::BROKER_CONFIG_SECTION;

pub use task::QueueError;
pub use task::Task;
pub use task::TaskHandle;
pub use task::TaskId;
pub use task::TaskQueue;
pub use task::TaskQueueFactory;
pub use task::TaskQueueFactoryContract;

#[cfg(feature = "tokio-rt")]
pub use task::InMemoryTaskQueue;

pub use validator_svc::Validator;
pub use validator_svc::VALIDATOR_SVC;
