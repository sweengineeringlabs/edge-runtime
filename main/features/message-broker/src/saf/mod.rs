//! SAF layer — message broker public facade.

mod edge_message_broker_svc;

#[cfg(feature = "tokio-rt")]
pub use edge_message_broker_svc::in_memory_broker;
#[cfg(feature = "nats")]
pub use edge_message_broker_svc::nats_broker;
