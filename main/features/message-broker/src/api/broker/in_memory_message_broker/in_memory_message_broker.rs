//! API interface surface for the in-memory broker implementation.
//!
//! This is the SEA api/ counterpart for the in-memory broker.
//! The implementation is accessed via [`crate::in_memory_broker`].

/// API marker type identifying the in-memory broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::in_memory_broker`] which returns `impl MessageBroker`.
pub struct InMemoryMessageBroker;
