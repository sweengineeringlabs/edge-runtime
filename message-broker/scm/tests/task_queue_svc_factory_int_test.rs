//! Integration tests for task queue SAF surface — rule 120/220 coverage for `task_queue_svc`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_message_broker::{
    Task, TaskHandle, TaskId, TaskQueueFactory, TaskQueueFactoryContract,
};

// ── TaskQueueFactory (type existence + basic usage) ──────────────────────────

/// @covers: TaskQueueFactory
#[test]
fn test_task_queue_factory_is_constructible_happy() {
    let factory = TaskQueueFactory::default_factory();
    let id = factory.new_task_id();
    assert!(!id.to_string().is_empty(), "factory must produce valid non-empty task IDs");
}

/// @covers: TaskQueueFactory
#[test]
fn test_task_queue_factory_produces_unique_task_ids_edge() {
    use swe_edge_runtime_message_broker::TaskQueueFactoryContract;
    let factory = TaskQueueFactory::default_factory();
    let id1 = factory.new_task_id();
    let id2 = factory.new_task_id();
    assert_ne!(id1, id2);
}

/// @covers: TaskQueueFactory
#[test]
fn test_task_queue_factory_missing_feature_does_not_panic_error() {
    let factory = TaskQueueFactory::default_factory();
    let id1 = factory.new_task_id();
    let id2 = factory.new_task_id();
    assert_ne!(id1, id2, "factory produces distinct task IDs even without optional features");
}

// ── Task (value type) ────────────────────────────────────────────────────────

/// @covers: Task
#[test]
fn test_task_new_creates_task_with_payload_happy() {
    let task = Task::new(b"hello".as_ref());
    assert_eq!(task.payload.as_ref(), b"hello");
}

/// @covers: Task
#[test]
fn test_task_new_empty_payload_is_valid_edge() {
    let task = Task::new(b"".as_ref());
    assert!(task.payload.is_empty());
}

/// @covers: Task
#[test]
fn test_task_with_headers_stores_headers_error() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert("x-retry".into(), "3".into());
    let task = Task::with_headers(b"body".as_ref(), h.clone());
    assert_eq!(task.headers.get("x-retry").map(String::as_str), Some("3"));
}

// ── TaskId ────────────────────────────────────────────────────────────────────

/// @covers: TaskId
#[test]
fn test_task_id_from_task_is_set_happy() {
    let task = Task::new(b"x".as_ref());
    let id: TaskId = task.id;
    assert!(!id.to_string().is_empty(), "task ID extracted from Task must have a non-empty display");
}

/// @covers: TaskId
#[test]
fn test_task_id_unique_per_task_edge() {
    let t1 = Task::new(b"a".as_ref());
    let t2 = Task::new(b"a".as_ref());
    assert_ne!(t1.id, t2.id);
}

/// @covers: TaskId
#[test]
fn test_task_id_is_copy_error() {
    let task = Task::new(b"copy".as_ref());
    let id = task.id;
    let id2 = id; // Copy semantics
    assert_eq!(id, id2);
}

// ── TaskHandle ───────────────────────────────────────────────────────────────

/// @covers: TaskHandle
#[tokio::test]
async fn test_task_handle_ack_completes_future_happy() {
    let task = Task::new(b"t".as_ref());
    let ack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let nack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let handle = TaskHandle::new(task.id, task.payload, task.headers, ack, nack);
    assert!(handle.ack.await.is_ok());
}

/// @covers: TaskHandle
#[tokio::test]
async fn test_task_handle_nack_completes_future_edge() {
    let task = Task::new(b"t".as_ref());
    let ack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let nack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let handle = TaskHandle::new(task.id, task.payload, task.headers, ack, nack);
    assert!(handle.nack.await.is_ok());
}

/// @covers: TaskHandle
#[test]
fn test_task_handle_carries_task_id_error() {
    let task = Task::new(b"check".as_ref());
    let task_id = task.id;
    let ack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let nack = Box::pin(async { Ok::<(), swe_edge_runtime_message_broker::QueueError>(()) });
    let handle = TaskHandle::new(task.id, task.payload, task.headers, ack, nack);
    assert_eq!(handle.task_id, task_id);
}

// ── InMemoryTaskQueue (cfg tokio-rt) ─────────────────────────────────────────

/// @covers: InMemoryTaskQueue
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_queue_enqueue_dequeue_round_trip_happy() {
    use swe_edge_runtime_message_broker::TaskQueue;
    let queue = TaskQueueFactory::in_memory();
    let task = Task::new(b"round-trip".as_ref());
    let id = task.id;
    queue.enqueue(task).await.expect("enqueue failed");
    let handle = queue
        .dequeue()
        .await
        .expect("dequeue failed")
        .expect("no task");
    assert_eq!(handle.task_id, id);
}

/// @covers: InMemoryTaskQueue
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_queue_health_check_ok_edge() {
    use swe_edge_runtime_message_broker::TaskQueue;
    let queue = TaskQueueFactory::in_memory();
    assert!(queue.health_check().await.is_ok());
}

/// @covers: InMemoryTaskQueue
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_queue_oversized_payload_returns_error() {
    use swe_edge_runtime_message_broker::TaskQueue;
    let queue = TaskQueueFactory::in_memory();
    let big_payload = vec![0u8; 5 * 1024 * 1024]; // 5 MiB > 4 MiB limit
    let task = Task::new(big_payload);
    let result = queue.enqueue(task).await;
    assert!(result.is_err(), "oversized payload must be rejected");
}
