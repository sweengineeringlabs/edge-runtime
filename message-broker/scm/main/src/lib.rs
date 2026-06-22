//! `swe_edge_runtime_message_broker` — concrete broker backends + task queue.
//!
//! Provides the concrete backends over the `swe-edge-message-broker` contract.
//! Use [`MessageBrokerFactory::in_memory`] for testing and local services,
//! [`MessageBrokerFactory::nats`] for NATS-backed production deployments, and
//! [`MessageBrokerFactory::from_config`] to build a broker from configuration.
//!
//! Single entry point: `gateway::message_broker_svc`.

// `unwrap`/`expect` are denied in production code (workspace lints) but are the
// idiomatic assertion mechanism inside inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use crate::api::*;
pub use crate::saf::*;
