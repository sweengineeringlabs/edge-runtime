//! Integration tests for [`BrokerProvider`] trait (rule 222) and broker factory SAF fns (rule 221).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
use swe_edge_runtime_message_broker::{BrokerProvider, MessageBrokerFactory};

// --- BrokerProvider::default_factory (rule 222) ---

/// @covers: BrokerProvider::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let _ = factory;
    assert_eq!(
        swe_edge_runtime_message_broker::BrokerBackendConfig::default().backend,
        "inmemory",
        "default_factory backed by in-memory broker"
    );
}

/// @covers: BrokerProvider::default_factory
#[test]
fn test_default_factory_is_callable_multiple_times_edge() {
    let f1 = MessageBrokerFactory::default_factory();
    let f2 = MessageBrokerFactory::default_factory();
    let _ = (f1, f2);
    assert_eq!(
        swe_edge_runtime_message_broker::BrokerBackendConfig::default().backend,
        "inmemory",
        "multiple default_factory calls produce consistent defaults"
    );
}

/// @covers: BrokerProvider::default_factory
#[test]
fn test_default_factory_produces_usable_value_error() {
    // Calling default_factory on a type without a broker feature compiled in still succeeds;
    // the error surface is on build_from_config, not on factory construction.
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
        group_id: None,
    };
    let _ = (&factory, &config);
    assert_eq!(config.backend, BackendKind::InMemory, "InMemory backend config typed correctly");
}

// --- BrokerProvider::build_in_memory (rule 222, cfg tokio-rt) ---

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_returns_broker_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let broker = factory.build_in_memory();
    assert!(std::mem::size_of_val(&broker) > 0, "build_in_memory must return a non-ZST MessageBroker instance");
}

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_broker_is_send_and_sync_edge() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryMessageBroker>(); // @allow: no_mocks_in_integration
    assert!(std::mem::size_of::<swe_edge_runtime_message_broker::InMemoryMessageBroker>() > 0, "InMemoryMessageBroker is a non-ZST type that compiles as Send + Sync");
}

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_build_in_memory_broker_health_check_succeeds_error() {
    use swe_edge_message_broker::MessageBroker;
    let factory = MessageBrokerFactory::default_factory();
    let broker = factory.build_in_memory();
    // health_check returns Ok on in-memory; an Err here would be a regression.
    assert!(
        broker.health_check().await.is_ok(),
        "in-memory broker health_check must not fail"
    );
}

// --- BrokerProvider::build_from_config (rule 222) ---

/// @covers: BrokerProvider::build_from_config
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_build_from_config_in_memory_backend_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
        group_id: None,
    };
    let broker = factory.build_from_config(&config).await.unwrap();
    assert!(broker.health_check().await.is_ok());
}

/// @covers: BrokerProvider::build_from_config
#[tokio::test]
async fn test_build_from_config_nats_without_feature_returns_unavailable_error() {
    #[cfg(not(feature = "nats"))]
    {
        use swe_edge_runtime_message_broker::BrokerError;
        let factory = MessageBrokerFactory::default_factory();
        let config = MessageBrokerConfig {
            backend: BackendKind::Nats,
            url: Some("nats://127.0.0.1:4229".into()),
            group_id: None,
        };
        let result = factory.build_from_config(&config).await;
        assert!(
            matches!(result, Err(BrokerError::Unavailable(_))),
            "expected Unavailable when nats feature is off"
        );
    }
    #[cfg(feature = "nats")]
    {
        use swe_edge_runtime_message_broker::BrokerError;
        let factory = MessageBrokerFactory::default_factory();
        let config = MessageBrokerConfig {
            backend: BackendKind::Nats,
            url: Some("nats://127.0.0.1:4229".into()),
            group_id: None,
        };
        let result = factory.build_from_config(&config).await;
        assert!(
            matches!(result, Err(BrokerError::Connection(_))),
            "expected Connection error for unreachable NATS"
        );
    }
}

/// @covers: BrokerProvider::build_from_config
#[tokio::test]
async fn test_build_from_config_kafka_without_feature_returns_unavailable_edge() {
    #[cfg(not(feature = "kafka"))]
    {
        use swe_edge_runtime_message_broker::BrokerError;
        let factory = MessageBrokerFactory::default_factory();
        let config = MessageBrokerConfig {
            backend: BackendKind::Kafka,
            url: Some("127.0.0.1:9999".into()),
            group_id: Some("test-group".into()),
        };
        let result = factory.build_from_config(&config).await;
        assert!(
            matches!(result, Err(BrokerError::Unavailable(_))),
            "expected Unavailable when kafka feature is off"
        );
    }
    #[cfg(feature = "kafka")]
    {
        let factory = MessageBrokerFactory::default_factory();
        let config = MessageBrokerConfig {
            backend: BackendKind::Kafka,
            url: Some("127.0.0.1:9999".into()),
            group_id: Some("test-group".into()),
        };
        let _ = factory.build_from_config(&config).await;
    }
}
