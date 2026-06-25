//! SAF layer — internal implementation.

mod broker;
mod config_provider_svc_factory;
mod task;
mod validator_svc_factory;

pub use broker::DEFAULT_BROKER_BACKEND;
pub use config_provider_svc_factory::BROKER_CONFIG_SECTION;
pub use task::MAX_TASK_PAYLOAD_BYTES;
pub use task::TASK_QUEUE_FACTORY_CONTRACT_ID;
pub use validator_svc_factory::VALIDATOR_SVC;
