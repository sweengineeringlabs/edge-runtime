//! Integration tests for the Kafka message broker.
//!
//! These tests run against a synthetic unreachable broker to verify error paths.
//! Tests that require a live Kafka cluster are skipped when none is available.

#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: MessageBrokerFactory::kafka — construction succeeds even for an unreachable host.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_message_broker_factory_constructs_without_network() {
    use swe_edge_runtime_message_broker::MessageBrokerFactory;
    // rdkafka connects lazily — construction must not panic or error.
    let result = MessageBrokerFactory::kafka("127.0.0.1:9999", "test-group");
    assert!(
        result.is_ok(),
        "Kafka broker factory must succeed before the first IO attempt"
    );
}

/// @covers: MessageBrokerFactory::kafka — health_check fails for an unreachable broker.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_message_broker_health_check_fails_for_unreachable_broker() {
    use swe_edge_message_broker::{BrokerError, MessageBroker as _};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let broker =
        MessageBrokerFactory::kafka("127.0.0.1:9999", "test-group").expect("client builds");
    let result = broker.health_check().await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "health_check must return Connection error for unreachable broker"
    );
}

/// @covers: MessageBrokerFactory::kafka — publish fails for an unreachable broker.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_message_broker_publish_fails_for_unreachable_broker() {
    use swe_edge_message_broker::{BrokerError, Message, MessageBroker as _};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let broker =
        MessageBrokerFactory::kafka("127.0.0.1:9999", "test-group").expect("client builds");
    let result = broker
        .publish("topic", Message::new(b"payload".as_ref()))
        .await;
    assert!(
        matches!(result, Err(BrokerError::Publish { .. })),
        "publish must return Publish error for unreachable broker"
    );
}

/// @covers: MessageBrokerFactory::from_config — kafka backend without url is rejected.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_from_config_kafka_without_url_returns_connection_error() {
    use swe_edge_message_broker::{BackendKind, BrokerError, MessageBrokerConfig};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: None,
        group_id: Some("workers".into()),
    };
    let result = MessageBrokerFactory::from_config(&cfg).await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "from_config without url must return Connection error"
    );
}

/// @covers: MessageBrokerFactory::from_config — kafka backend without group_id is rejected.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_from_config_kafka_without_group_id_returns_connection_error() {
    use swe_edge_message_broker::{BackendKind, BrokerError, MessageBrokerConfig};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: Some("127.0.0.1:9999".into()),
        group_id: None,
    };
    let result = MessageBrokerFactory::from_config(&cfg).await;
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "from_config without group_id must return Connection error"
    );
}

// ── Live-broker tests ────────────────────────────────────────────────────────
//
// Run with a real Kafka cluster:
//
//   KAFKA_BROKERS=localhost:9092 cargo test --features kafka -- \
//     --include-ignored --test kafka_message_broker_int_test
//
// All tests below are ignored unless explicitly included so normal CI (without
// a Kafka sidecar) still passes green.

/// Returns the broker address from KAFKA_BROKERS, or panics with a clear message.
#[cfg(feature = "kafka")]
fn require_kafka_brokers() -> String {
    std::env::var("KAFKA_BROKERS")
        .expect("KAFKA_BROKERS env var must be set to run live-broker tests (e.g. localhost:9092)")
}

/// @covers: publish + subscribe — happy-path roundtrip with a live broker.
///
/// Publishes one message and verifies the subscriber stream yields it with the
/// same payload. Also exercises the bounded channel created by `subscribe`.
#[cfg(feature = "kafka")]
#[tokio::test]
#[ignore = "requires-kafka"]
async fn test_publish_subscribe_roundtrip_with_live_broker() {
    use futures::StreamExt as _;
    use swe_edge_message_broker::{Message, MessageBroker as _};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let brokers = require_kafka_brokers();
    let topic = "swe-edge-test-pub-sub-roundtrip";
    let payload = b"live-broker-payload";

    let broker = MessageBrokerFactory::kafka(&brokers, "swe-edge-test-group-sub")
        .expect("broker construction must succeed with a live broker");

    // Subscribe before publishing so the consumer is assigned the partition first.
    let mut stream = broker
        .subscribe(topic)
        .await
        .expect("subscribe must succeed with a live broker");

    // Brief pause to let the consumer group rebalance complete.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    broker
        .publish(topic, Message::new(payload.as_ref()))
        .await
        .expect("publish must succeed with a live broker");

    // Receive with a timeout so the test doesn't hang if the message is dropped.
    let received = tokio::time::timeout(std::time::Duration::from_secs(10), stream.next())
        .await
        .expect("message must arrive within 10 s")
        .expect("stream must not end before yielding a message")
        .expect("stream item must not be an error");

    assert_eq!(
        received.payload.as_ref(),
        payload,
        "received payload must match the published payload"
    );
}

/// @covers: subscribe — bounded channel construction does not panic or deadlock.
///
/// Verifies that `subscribe` returns a valid stream object without hanging, which
/// proves the bounded channel was created with a positive capacity. Full backpressure
/// verification (filling the channel until the poll loop yields) requires sustained
/// produce throughput and is covered by the live-broker suite.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_subscribe_returns_stream_without_panicking() {
    use swe_edge_message_broker::MessageBroker as _;
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    let broker = MessageBrokerFactory::kafka("127.0.0.1:9999", "test-group-cap")
        .expect("construction must succeed before first IO");

    // subscribe() constructs the bounded channel; if capacity were 0 this would deadlock.
    // With an unreachable broker the channel is created but no messages will arrive.
    let result = broker.subscribe("test-topic-cap").await;
    assert!(
        result.is_ok(),
        "subscribe must succeed (channel construction, not network)"
    );
}
