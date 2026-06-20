//! [`MessageBrokerFactory`] — zero-size factory type for constructing broker instances.

/// Zero-size factory type for constructing message broker instances.
///
/// All factory methods are associated functions on this type.
/// Consumers never construct `MessageBrokerFactory` directly — they call
/// associated functions like `MessageBrokerFactory::in_memory()`.
pub struct MessageBrokerFactory;
