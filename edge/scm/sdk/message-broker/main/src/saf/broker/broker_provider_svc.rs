//! SAF — [`BrokerProvider`] public service surface.
//!
//! Exposes the [`BrokerProvider`] trait for consumers that need to construct
//! [`swe_edge_message_broker::MessageBroker`] instances from a factory type.

pub use crate::api::BrokerProvider;

/// Default backend identifier used when no configuration override is provided.
pub const DEFAULT_BROKER_BACKEND: &str = "inmemory";
