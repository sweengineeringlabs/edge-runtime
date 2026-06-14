//! Broker theme port contracts.
//!
//! [`BrokerFactory`] is the native trait declared in this module.
//! The [`MessageBroker`], [`Message`], and [`MessageStream`] types are re-exported
//! from the `swe-edge-message-broker` contract crate.

pub mod broker_factory;

pub use swe_edge_message_broker::{Message, MessageBroker, MessageStream};
