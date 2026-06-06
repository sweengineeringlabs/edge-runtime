//! Broker port contract.
//!
//! The [`MessageBroker`] trait and its [`Message`]/[`MessageStream`] value types
//! are owned by the `swe-edge-message-broker` contract crate and re-exported here
//! so backends and the factory import them from a single theme path.

pub use swe_edge_message_broker::{Message, MessageBroker, MessageStream};
