//! API interface surface for the NATS broker implementation.
//!
//! This is the SEA api/ counterpart for
//! `core/broker/nats_message_broker/nats_message_broker.rs`.
//! The implementation is accessed via [`crate::nats_broker`].

#[allow(dead_code)]
/// API marker type identifying the NATS broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::nats_broker`] which returns `impl MessageBroker`.
pub struct NatsMessageBroker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nats_message_broker_api_marker_is_constructible() {
        let _ = NatsMessageBroker;
    }
}
