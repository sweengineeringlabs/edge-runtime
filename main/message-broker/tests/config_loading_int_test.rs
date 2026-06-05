//! Integration tests for [`MessageBrokerFactory::from_config`].

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(feature = "tokio-rt")]
use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
#[cfg(feature = "tokio-rt")]
use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: from_config — an in_memory config builds a real, usable broker that
/// round-trips a published message.
#[tokio::test]
#[cfg(feature = "tokio-rt")]
async fn test_from_config_in_memory_builds_usable_broker() {
    use futures::StreamExt as _;
    use swe_edge_message_broker::Message;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
    };
    let broker = MessageBrokerFactory::from_config(&cfg)
        .await
        .expect("in_memory broker builds with tokio-rt");

    let mut sub = broker.subscribe("topic").await.expect("subscribe");
    broker
        .publish("topic", Message::new(b"hello".as_ref()))
        .await
        .expect("publish");
    let received = sub
        .next()
        .await
        .expect("a message is delivered")
        .expect("delivery is not an error");
    assert_eq!(&received.payload[..], b"hello");
}
