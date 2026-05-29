//! API interface surface for the NATS broker implementation.
//!
//! This is the SEA api/ counterpart for the NATS broker.
//! The implementation is accessed via [`crate::nats_broker`].

/// API marker type identifying the NATS broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::nats_broker`] which returns `impl MessageBroker`.
pub struct NatsMessageBroker;
