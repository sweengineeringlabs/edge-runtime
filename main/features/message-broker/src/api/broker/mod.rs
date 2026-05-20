//! Broker API — traits, value types, and error definitions.

pub(crate) mod broker_error;
#[cfg(feature = "tokio-rt")]
pub(crate) mod in_memory_message_broker;
pub(crate) mod message;
pub(crate) mod message_broker;
pub(crate) mod message_stream;
#[cfg(feature = "nats")]
pub(crate) mod nats_message_broker;

pub use broker_error::BrokerError;
pub use message::message::Message;
pub use message_broker::MessageBroker;
pub use message_stream::MessageStream;
