//! API marker type for the in-memory broker implementation.

/// API marker type identifying the in-memory broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::MessageBrokerFactory::in_memory`] which returns `impl MessageBroker`.
pub struct InMemoryMessageBroker;
