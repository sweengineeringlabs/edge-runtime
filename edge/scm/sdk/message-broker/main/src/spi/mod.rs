//! SPI — external-library implementations of the broker and task contracts.
//!
//! Mirrors the `api/` theme structure as `spi/{theme}/{technology}/`. Each
//! backend (Kafka, NATS) wraps an external client library behind the
//! technology-neutral `api/` ports. In-house backends (`InMemory*`) live in
//! `api/{theme}/types/`, not here.
//!
//! **Note**: all provider modules are private. Consumers never name
//! `KafkaMessageBroker`, `NatsMessageBroker`, or any other implementation type;
//! `saf/` exposes only factories returning `impl Trait` / `Box<dyn Trait>`.

pub(crate) mod broker;
pub(crate) mod task;

#[cfg(feature = "kafka")]
pub(crate) use broker::kafka::KafkaMessageBroker;
#[cfg(feature = "nats")]
pub(crate) use broker::nats::NatsMessageBroker;
#[cfg(feature = "kafka")]
pub(crate) use task::kafka::KafkaTaskQueue;
#[cfg(feature = "nats")]
pub(crate) use task::nats::NatsTaskQueue;
