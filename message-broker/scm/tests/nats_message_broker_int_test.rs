//! Integration tests for the NATS message broker API marker.

/// @covers: MessageBrokerFactory::nats
#[cfg(feature = "nats")]
#[tokio::test]
async fn test_nats_message_broker_connect_fails_for_unreachable_host() {
    use swe_edge_runtime_message_broker::{BrokerError, MessageBrokerFactory};
    let result = MessageBrokerFactory::nats("nats://127.0.0.1:4229").await;
    assert!(matches!(result, Err(BrokerError::Connection(_))));
}
