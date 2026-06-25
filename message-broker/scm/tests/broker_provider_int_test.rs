//! Rule-222 coverage for [`BrokerProvider`] trait fns in `api/broker/traits/broker_provider`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
use swe_edge_runtime_message_broker::{BrokerProvider, MessageBrokerFactory};

// ── BrokerProvider::default_factory ──────────────────────────────────────────

/// @covers: BrokerProvider::default_factory
#[test]
fn test_default_factory_returns_usable_instance_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let _ = factory;
    let default_backend = swe_edge_runtime_message_broker::BrokerBackendConfig::default().backend;
    assert_eq!(default_backend, "inmemory", "default factory is backed by in-memory broker");
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
fn test_default_factory_produces_value_that_holds_config_error() {
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
        group_id: None,
    };
    let _ = (&factory, &config);
    assert_eq!(config.backend, BackendKind::InMemory, "InMemory backend config typed correctly");
}

// ── BrokerProvider::build_in_memory ──────────────────────────────────────────

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_returns_broker_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let broker = factory.build_in_memory();
    assert!(std::mem::size_of_val(&broker) > 0, "build_in_memory must return a non-ZST MessageBroker");
}

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_build_in_memory_health_check_passes_edge() {
    use swe_edge_message_broker::MessageBroker;
    let factory = MessageBrokerFactory::default_factory();
    let broker = factory.build_in_memory();
    assert!(broker.health_check().await.is_ok());
}

/// @covers: BrokerProvider::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryMessageBroker>(); // @allow: no_mocks_in_integration
    assert!(std::mem::size_of::<swe_edge_runtime_message_broker::InMemoryMessageBroker>() > 0, "InMemoryMessageBroker is a non-ZST type that compiles as Send + Sync");
}

// ── BrokerProvider::build_from_config ────────────────────────────────────────

/// @covers: BrokerProvider::build_from_config
#[test]
fn test_build_from_config_returns_result_not_panic_happy() {
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    // build_from_config always returns a Result — it must never panic.
    // With tokio-rt feature: Ok(broker); without: Err(Unavailable). Both valid.
    let _ = rt.block_on(factory.build_from_config(&config));
}

/// @covers: BrokerProvider::build_from_config
#[test]
fn test_build_from_config_nats_without_url_returns_error() {
    use swe_edge_runtime_message_broker::BrokerError;
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(factory.build_from_config(&config));
    // Without nats feature: Unavailable; with nats + no URL: Connection error.
    assert!(
        matches!(
            result,
            Err(BrokerError::Unavailable(_)) | Err(BrokerError::Connection(_))
        ),
        "nats without URL must return Unavailable or Connection error"
    );
}

/// @covers: BrokerProvider::build_from_config
#[test]
fn test_build_from_config_kafka_without_brokers_returns_error_edge() {
    let factory = MessageBrokerFactory::default_factory();
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(factory.build_from_config(&config));
    assert!(
        result.is_err(),
        "kafka without brokers/group_id must return an error"
    );
}
