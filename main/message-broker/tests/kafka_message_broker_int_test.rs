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
