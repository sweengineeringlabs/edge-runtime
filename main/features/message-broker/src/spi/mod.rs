//! SPI — Service Provider Interface for message broker and task queue implementations.
//!
//! Providers extend and override core/ logic for specific backends.
//! Each backend (in-memory, NATS, etc.) implements the provider interface here.
//!
//! **Note**: All provider modules are private. Consumers never see InMemoryMessageBroker,
//! NatsMessageBroker, or any implementation types. SAF exposes only factories
//! that return `impl Trait`, hiding all implementation details.

#[cfg(feature = "tokio-rt")]
mod memory;

#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "tokio-rt")]
pub(crate) use memory::{InMemoryMessageBroker, InMemoryTaskQueue};

#[cfg(feature = "nats")]
pub(crate) use nats::{NatsMessageBroker, NatsTaskQueue};
