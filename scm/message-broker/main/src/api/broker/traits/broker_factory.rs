//! [`BrokerFactory`] — contract for types that construct [`MessageBroker`] instances.

use futures::future::BoxFuture;

use crate::api::broker::errors::BrokerError;
use crate::api::broker::types::message_broker_factory::MessageBrokerFactory;

/// Contract for types that construct [`swe_edge_message_broker::MessageBroker`] instances.
///
/// Implementors produce concrete broker instances from a factory type.
/// [`MessageBrokerFactory`] is the canonical implementor in this crate.
pub trait BrokerFactory {
    /// Construct an in-memory broker backend.
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(
        &self,
    ) -> crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;

    /// Construct a broker from configuration, dispatching to the appropriate backend.
    ///
    /// # Errors
    ///
    /// Returns [`BrokerError`] if the configured backend is unavailable or
    /// the backend connection cannot be established.
    fn build_from_config<'a>(
        &'a self,
        config: &'a swe_edge_message_broker::MessageBrokerConfig,
    ) -> BoxFuture<'a, Result<Box<dyn swe_edge_message_broker::MessageBroker>, BrokerError>>;

    /// Return the default [`MessageBrokerFactory`] instance for constructing brokers.
    fn default_factory() -> MessageBrokerFactory {
        MessageBrokerFactory
    }
}

impl BrokerFactory for MessageBrokerFactory {
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(
        &self,
    ) -> crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker {
        MessageBrokerFactory::in_memory()
    }

    fn build_from_config<'a>(
        &'a self,
        config: &'a swe_edge_message_broker::MessageBrokerConfig,
    ) -> BoxFuture<'a, Result<Box<dyn swe_edge_message_broker::MessageBroker>, BrokerError>> {
        Box::pin(MessageBrokerFactory::from_config(config))
    }
}
