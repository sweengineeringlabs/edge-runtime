//! Broker theme port contracts.
//!
//! [`BrokerProvider`] is the native trait declared in this module.
//! The [`MessageBroker`], [`Message`], and [`MessageStream`] types are re-exported
//! from the `swe-edge-message-broker` contract crate.

pub mod broker_provider;

pub use swe_edge_message_broker::{Message, MessageBroker, MessageStream};
