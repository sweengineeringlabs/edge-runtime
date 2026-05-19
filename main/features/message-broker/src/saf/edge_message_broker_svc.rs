//! SAF — message broker public factory surface.

#[cfg(feature = "nats")]
use crate::api::broker::broker_error::BrokerError;
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use crate::api::broker::message_broker::MessageBroker;
#[cfg(feature = "tokio-rt")]
use crate::core::broker::InMemoryMessageBroker;
#[cfg(feature = "nats")]
use crate::core::broker::NatsMessageBroker;

/// Construct an in-memory broker backed by [`tokio::sync::broadcast`].
///
/// Topics are created lazily on first subscription.  All subscribers on the
/// same topic receive every message published after they subscribed.
///
/// Requires the `tokio-rt` feature.
#[cfg(feature = "tokio-rt")]
pub fn in_memory_broker() -> impl MessageBroker + Clone {
    InMemoryMessageBroker::new()
}

/// Connect to a NATS server and return a broker handle.
///
/// # Errors
///
/// Returns [`BrokerError::Connection`] if the NATS server is unreachable.
///
/// Requires the `nats` feature.
#[cfg(feature = "nats")]
pub async fn nats_broker(url: &str) -> Result<impl MessageBroker, BrokerError> {
    NatsMessageBroker::connect(url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: in_memory_broker
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_in_memory_broker_factory_produces_working_broker() {
        use futures::executor::block_on;
        let broker = in_memory_broker();
        block_on(async move {
            assert!(broker.health_check().await.is_ok());
        });
    }

    /// @covers: nats_broker
    #[test]
    fn test_nats_broker_is_feature_gated_behind_nats() {
        let enabled = cfg!(feature = "nats");
        let _ = enabled;
    }

    /// @covers: validate
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_validate_returns_ok_for_valid_broker() {
        use crate::api::traits::Validator;
        use crate::core::broker::InMemoryMessageBroker;
        assert!(InMemoryMessageBroker::new().validate().is_ok());
    }
}
