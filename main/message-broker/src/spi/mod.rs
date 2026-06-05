//! SPI — Service Provider Interface for message broker and task queue implementations.
//!
//! Providers extend and override core/ logic for specific backends.
//! Each backend (NATS, etc.) implements the provider interface here.
//! In-memory implementations live in `api/` for direct naming by callers.
//!
//! **Note**: All provider modules are private. Consumers never see
//! NatsMessageBroker or any implementation types. SAF exposes only factories
//! that return concrete types, hiding all implementation details.

#[cfg(feature = "kafka")]
mod kafka;
#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "kafka")]
pub(crate) use kafka::{KafkaMessageBroker, KafkaTaskQueue};
#[cfg(feature = "nats")]
pub(crate) use nats::{NatsMessageBroker, NatsTaskQueue};
