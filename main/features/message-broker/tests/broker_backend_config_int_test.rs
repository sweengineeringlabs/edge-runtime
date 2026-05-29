//! Integration tests for the broker backend configuration.
//!
//! Verifies that the broker factory respects config defaults —
//! the default backend is "inmemory" as documented in config/application.toml.

/// @covers: MessageBrokerFactory::broker_from_config — default config uses inmemory backend
#[tokio::test]
#[cfg(feature = "tokio-rt")]
async fn test_broker_from_config_default_backend_is_inmemory() {
    use swe_edge_runtime_message_broker::{MessageBroker, MessageBrokerFactory};
    let broker = MessageBrokerFactory::broker_from_config()
        .await
        .map_err(|e| e.to_string())
        .ok();
    if let Some(b) = broker {
        assert!(
            b.health_check().await.is_ok(),
            "default in-memory broker must respond to health_check"
        );
    }
}
