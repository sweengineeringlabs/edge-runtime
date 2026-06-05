//! Broker API — backend implementations over the `swe-edge-message-broker` contract.
//!
//! The [`MessageBroker`] trait and the [`Message`]/[`MessageStream`]/[`BrokerError`]
//! value types are owned by the `swe-edge-message-broker` contract crate and are
//! re-exported here for convenience. This crate supplies the concrete backends
//! (in-memory tokio broadcast, NATS) plus the `from_config` construction factory.

#[cfg(feature = "tokio-rt")]
pub(crate) mod memory;
#[cfg(feature = "nats")]
pub(crate) mod nats;
pub(crate) mod types;

pub use swe_edge_message_broker::{BrokerError, Message, MessageBroker, MessageStream};
