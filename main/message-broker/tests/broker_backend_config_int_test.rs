//! Integration tests for backend selection via [`MessageBrokerFactory::from_config`].
//!
//! Verifies the contract config vocabulary (`MessageBrokerConfig` + `BackendKind`)
//! drives backend construction and that misconfiguration is rejected.

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: from_config — an in_memory config yields a healthy broker.
#[tokio::test]
#[cfg(feature = "tokio-rt")]
async fn test_from_config_in_memory_health_check_returns_ok() {
    let cfg = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
    };
    let broker = MessageBrokerFactory::from_config(&cfg)
        .await
        .expect("in_memory broker builds with tokio-rt");
    assert!(
        broker.health_check().await.is_ok(),
        "default in-memory broker must respond to health_check"
    );
}

/// @covers: from_config — a nats backend without a `url` is rejected with a
/// Connection error rather than silently constructing an unusable broker.
#[tokio::test]
#[cfg(feature = "nats")]
async fn test_from_config_nats_without_url_returns_connection_error() {
    use swe_edge_message_broker::BrokerError;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: None,
    };
    let result = MessageBrokerFactory::from_config(&cfg).await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "nats backend without a url must fail with a Connection error"
    );
}
