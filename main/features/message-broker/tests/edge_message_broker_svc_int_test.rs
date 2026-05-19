//! Public-API integration tests for the message broker SAF surface.

use swe_edge_message_broker::MessageBroker;

/// @covers: in_memory_broker
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_health_check_returns_ok() {
    use swe_edge_message_broker::in_memory_broker;
    assert!(in_memory_broker().health_check().await.is_ok());
}

/// @covers: in_memory_broker
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_pub_sub_roundtrip() {
    use bytes::Bytes;
    use futures::StreamExt as _;
    use swe_edge_message_broker::{in_memory_broker, Message};

    let broker = in_memory_broker();
    let mut stream = broker.subscribe("svc-test").await.unwrap();
    broker
        .publish("svc-test", Message::new(b"ping".as_ref()))
        .await
        .unwrap();
    let msg = stream.next().await.unwrap().unwrap();
    assert_eq!(msg.payload, Bytes::from_static(b"ping"));
}

/// @covers: nats_broker
#[cfg(feature = "nats")]
#[tokio::test]
async fn test_nats_broker_returns_connection_error_for_unreachable_host() {
    use swe_edge_message_broker::{nats_broker, BrokerError};
    let result = nats_broker("nats://127.0.0.1:4229").await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "expected Connection error, got: {result:?}"
    );
}
