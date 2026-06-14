//! Broker theme — message broker port, value types, and errors.
//!
//! The [`BrokerFactory`] trait is declared natively in this crate's `traits/`.
//! The [`MessageBroker`] trait and the [`Message`]/[`MessageStream`]/[`BrokerError`]
//! value types are owned by the `swe-edge-message-broker` contract crate and are
//! re-exported through this theme's `traits`/`error` dirs for a single import
//! path. This crate supplies the concrete backends — the in-house in-memory
//! broker (`types/`) plus the Kafka/NATS wrappers in `spi/broker/`.

pub(crate) mod broker_validation_result;
pub(crate) mod errors;
pub(crate) mod traits;
pub(crate) mod types;
pub(crate) mod validator;

pub use errors::BrokerError;
pub use traits::{Message, MessageBroker, MessageStream};
pub use types::MessageBrokerFactory;
