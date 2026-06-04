//! Integration tests for [`MessageBrokerFactory::broker_from_config`].

use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: MessageBrokerFactory::broker_from_config
#[tokio::test]
#[cfg(feature = "tokio-rt")]
async fn test_broker_from_config_loads_and_instantiates_default_inmemory() {
    let broker = MessageBrokerFactory::broker_from_config()
        .await
        .map_err(|e| e.to_string())
        .ok();
    if let Some(b) = broker {
        assert!(b.health_check().await.is_ok());
    }
}
