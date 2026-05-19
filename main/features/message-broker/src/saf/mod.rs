//! SAF layer — message broker public facade.

mod edge_message_broker_svc;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;

#[cfg(feature = "tokio-rt")]
pub use edge_message_broker_svc::in_memory_broker;
#[cfg(feature = "nats")]
pub use edge_message_broker_svc::nats_broker;
pub use edge_message_broker_svc::validate;
