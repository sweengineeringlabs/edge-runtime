//! Integration tests for [`TaskQueueFactoryContract`] trait (rule 222).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use bytes::Bytes;
use swe_edge_runtime_message_broker::{
    QueueError, TaskId, TaskQueueFactory, TaskQueueFactoryContract,
};

// --- TaskQueueFactoryContract::default_factory (rule 222) ---

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let factory = TaskQueueFactory::default_factory();
    let id = factory.new_task_id();
    assert!(!id.to_string().is_empty(), "default_factory produces a factory that generates valid task IDs");
}

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_is_callable_multiple_times_edge() {
    let f1 = TaskQueueFactory::default_factory();
    let f2 = TaskQueueFactory::default_factory();
    assert_ne!(f1.new_task_id(), f2.new_task_id(), "consecutive task IDs from multiple factory instances must be unique");
}

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_type_matches_expected_error() {
    let factory: TaskQueueFactory = TaskQueueFactory::default_factory();
    let id = factory.new_task_id();
    assert!(!id.to_string().is_empty(), "TaskQueueFactory contract: factory produces valid task IDs");
}

// --- TaskQueueFactoryContract::new_task_id (rule 222) ---

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_returns_unique_ids_happy() {
    let factory = TaskQueueFactory::default_factory();
    let id1 = factory.new_task_id();
    let id2 = factory.new_task_id();
    assert_ne!(id1, id2, "consecutive task IDs must be unique");
}

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_is_callable_on_factory_instance_edge() {
    let factory = TaskQueueFactory::default_factory();
    let id = factory.new_task_id();
    assert!(!id.to_string().is_empty(), "new_task_id produces a non-empty task ID string");
}

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_many_calls_all_unique_error() {
    let factory = TaskQueueFactory::default_factory();
    let ids: Vec<_> = (0..16).map(|_| factory.new_task_id()).collect();
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(
        ids.len(),
        unique.len(),
        "all generated task IDs must be unique"
    );
}

// --- TaskQueueFactoryContract::build_handle (rule 222) ---

/// @covers: TaskQueueFactoryContract::build_handle
#[test]
fn test_build_handle_returns_builder_with_correct_task_id_happy() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"payload");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskQueueFactory::build_handle(id, payload, HashMap::new(), ack, nack).build();
    assert_eq!(
        handle.task_id, id,
        "build_handle must seed the builder with the given task_id"
    );
}

/// @covers: TaskQueueFactoryContract::build_handle
#[test]
fn test_build_handle_empty_payload_produces_valid_handle_edge() {
    let id = TaskId::new();
    let payload = Bytes::new();
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskQueueFactory::build_handle(id, payload, HashMap::new(), ack, nack).build();
    assert!(
        handle.payload.is_empty(),
        "build_handle must accept empty payload"
    );
}

/// @covers: TaskQueueFactoryContract::build_handle
#[test]
fn test_build_handle_with_headers_stores_headers_error() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"data");
    let mut headers = HashMap::new();
    headers.insert("x-key".to_owned(), "v".to_owned());
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskQueueFactory::build_handle(id, payload, headers.clone(), ack, nack).build();
    assert_eq!(
        handle.headers, headers,
        "build_handle must forward headers to the resulting TaskHandle"
    );
}

// --- TaskQueueFactoryContract::build_in_memory (rule 222, cfg tokio-rt) ---

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_returns_queue_happy() {
    let factory = TaskQueueFactory::default_factory();
    let queue = factory.build_in_memory();
    assert!(std::mem::size_of_val(&queue) > 0, "build_in_memory must return a non-ZST TaskQueue");
}

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_build_in_memory_queue_health_check_passes_edge() {
    use swe_edge_runtime_message_broker::TaskQueue;
    let factory = TaskQueueFactory::default_factory();
    let queue = factory.build_in_memory();
    assert!(
        queue.health_check().await.is_ok(),
        "in-memory queue health_check must not fail at construction"
    );
}

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_queue_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryTaskQueue>(); // @allow: no_mocks_in_integration
    assert!(std::mem::size_of::<swe_edge_runtime_message_broker::InMemoryTaskQueue>() > 0, "InMemoryTaskQueue is a non-ZST type that compiles as Send + Sync");
}
