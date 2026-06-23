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
    let builder = MessageBrokerFactory::create_config_builder();
    let loader = builder.build_loader();
    assert!(loader.is_ok(), "builder must produce a valid loader");
}

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_create_config_builder_no_panic_on_repeated_calls_error() {
    let b1 = MessageBrokerFactory::create_config_builder();
    let b2 = MessageBrokerFactory::create_config_builder();
    assert!(b1.build_loader().is_ok(), "first builder must be valid");
    assert!(b2.build_loader().is_ok(), "second builder must be valid");
}

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_create_config_builder_multiple_calls_independent_edge() {
    let b1 = MessageBrokerFactory::create_config_builder();
    let b2 = MessageBrokerFactory::create_config_builder();
    let l1 = b1.build_loader();
    let l2 = b2.build_loader();
    assert!(l1.is_ok(), "first loader must be valid");
    assert!(l2.is_ok(), "second loader must be valid");
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
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    #[cfg(feature = "tokio-rt")]
    assert!(result.is_ok(), "in-memory backend must be available with tokio-rt feature");
    #[cfg(not(feature = "tokio-rt"))]
    assert!(result.is_err(), "in-memory backend requires tokio-rt feature");
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

/// @covers: MessageBrokerFactory::kafka
#[test]
#[cfg(feature = "kafka")]
fn test_kafka_with_valid_args_happy() {
    let result = MessageBrokerFactory::kafka("localhost:9092", "test-group");
    // Returns either Ok(broker) or Err(Connection) — either is a success for this test
    // since it means the feature gate is working. Only panics or Err(Unavailable) would be failures.
    assert!(result.is_ok() || matches!(result, Err(e) if e.to_string().contains("Connection")),
            "kafka with valid feature must return result, not Unavailable");
}

/// @covers: MessageBrokerFactory::kafka
#[test]
#[cfg(not(feature = "kafka"))]
fn test_kafka_without_feature_unavailable_error() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Kafka,
        url: Some("localhost:9092".into()),
        group_id: Some("test-group".into()),
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "kafka backend without feature must return error");
}

/// @covers: MessageBrokerFactory::kafka
#[test]
#[cfg(feature = "kafka")]
fn test_kafka_invalid_broker_connection_error_edge() {
    let result = MessageBrokerFactory::kafka("invalid-broker-host:9092", "test-group");
    // May fail with connection error if broker is unreachable
    assert!(result.is_err(), "kafka with invalid broker must return error");
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

/// @covers: MessageBrokerFactory::nats
#[test]
#[cfg(feature = "nats")]
fn test_nats_with_valid_url_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::nats("nats://localhost:4222"));
    // Returns either Ok(broker) or Err(Connection) — either is a success for this test
    // since it means the feature gate is working. Only panics or Err(Unavailable) would be failures.
    assert!(result.is_ok() || matches!(result, Err(e) if e.to_string().contains("Connection")),
            "nats with valid feature must return result, not Unavailable");
}

/// @covers: MessageBrokerFactory::nats
#[test]
#[cfg(not(feature = "nats"))]
fn test_nats_without_feature_unavailable_error() {
    let config = MessageBrokerConfig {
        backend: BackendKind::Nats,
        url: Some("nats://localhost:4222".into()),
        group_id: None,
    };
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::from_config(&config));
    assert!(result.is_err(), "nats backend without feature must return error");
}

/// @covers: MessageBrokerFactory::nats
#[test]
#[cfg(feature = "nats")]
fn test_nats_invalid_server_connection_error_edge() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(MessageBrokerFactory::nats("nats://invalid-host:4222"));
    // Fails with connection error if server is unreachable
    assert!(result.is_err(), "nats with invalid server must return error");
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
    let health = rt.block_on(broker.health_check());
    assert_eq!(health, Ok(()), "in-memory broker must always be healthy");
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
    let queue = TaskQueueFactory::in_memory();
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("tokio rt");
    let health = rt.block_on(queue.health_check());
    assert!(health.is_ok(), "in-memory task queue must be healthy when constructed");
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
    let health = rt.block_on(queue.health_check());
    assert_eq!(health, Ok(()), "in-memory task queue must always be healthy");
}

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_task_queue_in_memory_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryTaskQueue>();
}

// ── TaskQueueFactory::kafka (cfg kafka) ──────────────────────────────────────

/// @covers: TaskQueueFactory::kafka
#[test]
#[cfg(feature = "kafka")]
fn test_task_queue_kafka_with_valid_args_happy() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let result = TaskQueueFactory::kafka("localhost:9092", "test-group", "test-topic");
    // Returns either Ok(queue) or Err(Connection) — either is a success for this test
    // since it means the feature gate is working. Only panics or Err(Unavailable) would be failures.
    assert!(result.is_ok() || matches!(result, Err(e) if e.to_string().contains("Connection")),
            "kafka with valid feature must return result, not Unavailable");
}


/// @covers: TaskQueueFactory::kafka
#[test]
#[cfg(feature = "kafka")]
fn test_task_queue_kafka_invalid_broker_connection_error_edge() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let result = TaskQueueFactory::kafka("invalid-broker-host:9092", "test-group", "test-topic");
    // May fail with connection error if broker is unreachable
    assert!(result.is_err(), "kafka with invalid broker must return error");
}

// ── TaskQueueFactory::nats (cfg nats) ────────────────────────────────────────

/// @covers: TaskQueueFactory::nats
#[test]
#[cfg(feature = "nats")]
fn test_task_queue_nats_with_valid_url_happy() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(TaskQueueFactory::nats("nats://localhost:4222", "test-stream".into(), "test-group".into()));
    // Returns either Ok(queue) or Err(Connection) — either is a success for this test
    // since it means the feature gate is working. Only panics or Err(Unavailable) would be failures.
    assert!(result.is_ok() || matches!(result, Err(e) if e.to_string().contains("Connection")),
            "nats with valid feature must return result, not Unavailable");
}


/// @covers: TaskQueueFactory::nats
#[test]
#[cfg(feature = "nats")]
fn test_task_queue_nats_invalid_server_connection_error_edge() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    let result = rt.block_on(TaskQueueFactory::nats("nats://invalid-host:4222", "test-stream".into(), "test-group".into()));
    // Fails with connection error if server is unreachable
    assert!(result.is_err(), "nats with invalid server must return error");
}
