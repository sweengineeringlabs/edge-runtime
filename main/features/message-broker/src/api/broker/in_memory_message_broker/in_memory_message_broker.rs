//! API interface surface for the in-memory broker implementation.
//!
//! This is the SEA api/ counterpart for
//! `core/broker/in_memory_message_broker/in_memory_message_broker.rs`.
//! The implementation is accessed via [`crate::in_memory_broker`].

#[allow(dead_code)]
/// API marker type identifying the in-memory broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::in_memory_broker`] which returns `impl MessageBroker`.
pub struct InMemoryMessageBroker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_message_broker_api_marker_is_constructible() {
        let _ = InMemoryMessageBroker;
    }
}
