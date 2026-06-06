//! NATS message broker backend (async-nats-backed).

#[allow(clippy::module_inception)]
pub(crate) mod nats_message_broker;

pub(crate) use nats_message_broker::NatsMessageBroker;
