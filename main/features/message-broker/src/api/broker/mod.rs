//! Broker API — traits, value types, and error definitions.

pub(crate) mod broker_error;
#[cfg(feature = "tokio-rt")]
pub(crate) mod memory;
pub(crate) mod message;
pub(crate) mod message_broker;
#[cfg(feature = "nats")]
pub(crate) mod nats;
pub(crate) mod types;

pub use broker_error::BrokerError;
pub use message::MessageStream;
pub use message_broker::MessageBroker;
pub use types::message::Message;
