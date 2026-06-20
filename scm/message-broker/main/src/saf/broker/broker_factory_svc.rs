//! SAF — [`BrokerFactory`] public service surface.
//!
//! Exposes the [`BrokerFactory`] trait for consumers that need to construct
//! [`swe_edge_message_broker::MessageBroker`] instances from a factory type.

pub use crate::api::BrokerFactory;

/// Default backend identifier used when no configuration override is provided.
pub const DEFAULT_BROKER_BACKEND: &str = "inmemory";
