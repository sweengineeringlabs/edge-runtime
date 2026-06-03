//! Public-API integration tests for the message broker SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_message_broker::MessageBroker;

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_health_check_returns_ok() {
    use swe_edge_runtime_message_broker::MessageBrokerFactory;
    assert!(MessageBrokerFactory::in_memory()
        .health_check()
        .await
        .is_ok());
}

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_pub_sub_roundtrip() {
    use bytes::Bytes;
    use futures::StreamExt as _;
    use swe_edge_runtime_message_broker::{Message, MessageBrokerFactory};

    let broker = MessageBrokerFactory::in_memory();
    let mut stream = broker
        .subscribe("svc-test")
        .await
        .map_err(|e| e.to_string())
        .ok()
        .unwrap();
    broker
        .publish("svc-test", Message::new(b"ping".as_ref()))
        .await
        .map_err(|e| e.to_string())
        .ok();
    let msg = stream
        .next()
        .await
        .unwrap()
        .map_err(|e| e.to_string())
        .ok()
        .unwrap();
    assert_eq!(msg.payload, Bytes::from_static(b"ping"));
}

/// @covers: MessageBrokerFactory::nats
#[cfg(feature = "nats")]
#[tokio::test]
async fn test_nats_broker_returns_connection_error_for_unreachable_host() {
    use swe_edge_runtime_message_broker::{BrokerError, MessageBrokerFactory};
    let result = MessageBrokerFactory::nats("nats://127.0.0.1:4229").await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "expected Connection error, got: {result:?}"
    );
}
