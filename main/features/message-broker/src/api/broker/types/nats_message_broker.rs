//! API marker type for the NATS broker implementation.

/// API marker type identifying the NATS broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::MessageBrokerFactory::nats`] which returns `impl MessageBroker`.
pub struct NatsMessageBroker;
