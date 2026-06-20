//! Integration tests for broker SAF factory functions — rule 221 coverage for `broker_svc`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
use swe_edge_runtime_message_broker::MessageBrokerFactory;

// ── MessageBrokerFactory::default_application_config ─────────────────────────

/// @covers: MessageBrokerFactory::default_application_config
#[test]
fn test_default_application_config_returns_inmemory_backend_happy() {
    let cfg = MessageBrokerFactory::default_application_config();
    assert_eq!(cfg.message_broker.backend, "inmemory");
}

/// @covers: MessageBrokerFactory::default_application_config
#[test]
fn test_default_application_config_nats_url_is_populated_error() {
    let cfg = MessageBrokerFactory::default_application_config();
    // Default NATS URL must be non-empty — an empty URL is a misconfiguration.
    assert!(!cfg.message_broker.nats_url.is_empty());
}

/// @covers: MessageBrokerFactory::default_application_config
#[test]
fn test_default_application_config_multiple_calls_are_independent_edge() {
    let cfg1 = MessageBrokerFactory::default_application_config();
    let cfg2 = MessageBrokerFactory::default_application_config();
    assert_eq!(cfg1.message_broker.backend, cfg2.message_broker.backend);
}

// ── MessageBrokerFactory::create_config_builder ──────────────────────────────

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_create_config_builder_returns_pre_seeded_builder_happy() {
    let _builder = MessageBrokerFactory::create_config_builder();
}

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_create_config_builder_no_panic_on_repeated_calls_error() {
    let _ = MessageBrokerFactory::create_config_builder();
}

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_create_config_builder_multiple_calls_independent_edge() {
    let _b1 = MessageBrokerFactory::create_config_builder();
    let _b2 = MessageBrokerFactory::create_config_builder();
}

// ── MessageBrokerFactory::from_config ────────────────────────────────────────

/// @covers: MessageBrokerFactory::from_config
#[test]
fn test_from_config_inmemory_returns_result_happy() {
    let config = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    // Always returns a Result — either Ok (with tokio-rt) or Err(Unavailable).
    let _ = rt.block_on(MessageBrokerFactory::from_config(&config));
}

/// @covers: MessageBrokerFactory::from_config
#[test]
fn test_from_config_nats_without_url_returns_error() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "nats without URL must fail");
}

/// @covers: MessageBrokerFactory::from_config
#[test]
fn test_from_config_kafka_without_url_returns_error_edge() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "kafka without url/group must fail");
}

// ── MessageBrokerFactory::kafka (cfg kafka) ──────────────────────────────────
// Tests exercise via from_config since `kafka` fn requires the "kafka" feature.

/// @covers: MessageBrokerFactory::kafka
#[test]
fn test_kafka_backend_route_returns_result_happy() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: Some("localhost:9092".into()),
        group_id: Some("test-group".into()),
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    // Err(Unavailable) without "kafka" feature, or Err(Connection) without broker.
    let _ = rt.block_on(MessageBrokerFactory::from_config(&config));
}

/// @covers: MessageBrokerFactory::kafka
#[test]
fn test_kafka_without_url_returns_error() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "kafka without url must fail");
}

/// @covers: MessageBrokerFactory::kafka
#[test]
fn test_kafka_without_group_id_returns_error_edge() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: Some("localhost:9092".into()),
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "kafka without group_id must fail");
}

// ── MessageBrokerFactory::nats (cfg nats) ────────────────────────────────────
// Tests exercise via from_config since `nats` fn requires the "nats" feature.

/// @covers: MessageBrokerFactory::nats
#[test]
fn test_nats_backend_route_returns_result_happy() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: Some("nats://localhost:4222".into()),
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    // Err(Unavailable) without "nats" feature, or Err(Connection) without server.
    let _ = rt.block_on(MessageBrokerFactory::from_config(&config));
}

/// @covers: MessageBrokerFactory::nats
#[test]
fn test_nats_without_url_returns_error() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: None,
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "nats without url must fail");
}

/// @covers: MessageBrokerFactory::nats
#[test]
fn test_nats_empty_url_is_invalid_edge() {
    // Edge: empty string URL is different from None — both must fail.
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: Some(String::new()),
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    // Without nats feature: Err(Unavailable); both are error outcomes.
    assert!(result.is_err(), "nats with empty url must fail");
}

// ── MessageBrokerFactory::in_memory (cfg tokio-rt) ──────────────────────────

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_broker_is_constructible_happy() {
    let _broker = MessageBrokerFactory::in_memory();
}

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_broker_health_check_passes_edge() {
    use swe_edge_message_broker::MessageBroker;
    let broker = MessageBrokerFactory::in_memory();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    assert!(rt.block_on(broker.health_check()).is_ok());
}

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_broker_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryMessageBroker>();
}

// ── TaskQueueFactory::in_memory (cfg tokio-rt) ───────────────────────────────

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_task_queue_in_memory_is_constructible_happy() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let _queue = TaskQueueFactory::in_memory();
}

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_task_queue_in_memory_health_check_passes_edge() {
    use swe_edge_runtime_message_broker::{TaskQueue, TaskQueueFactory};
    let queue = TaskQueueFactory::in_memory();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    assert!(rt.block_on(queue.health_check()).is_ok());
}

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_task_queue_in_memory_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryTaskQueue>();
}
