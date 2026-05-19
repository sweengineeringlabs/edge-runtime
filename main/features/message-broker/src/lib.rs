//! `swe_edge_message_broker` — cross-process pub/sub broker.
//!
//! Provides a runtime-agnostic [`MessageBroker`] trait for cross-process
//! publish/subscribe messaging.  Use [`in_memory_broker`] for testing and
//! local services, [`nats_broker`] for NATS-backed production deployments.

mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::*;
