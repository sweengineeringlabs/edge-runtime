//! Integration tests for the Kafka task queue.
//!
//! These tests run against a synthetic unreachable broker to verify error paths.
//! Tests that require a live Kafka cluster are skipped when none is available.

#![allow(clippy::unwrap_used, clippy::expect_used)]

/// @covers: TaskQueueFactory::kafka — construction succeeds before first IO.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_task_queue_factory_constructs_without_network() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    // subscribe() is called during construction; rdkafka requires a tokio runtime for this.
    let result = TaskQueueFactory::kafka("127.0.0.1:9999", "test-group", "test-topic");
    assert!(
        result.is_ok(),
        "Kafka task queue factory must succeed before the first IO attempt"
    );
}

/// @covers: TaskQueueFactory::kafka — health_check fails for an unreachable broker.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_task_queue_health_check_fails_for_unreachable_broker() {
    use swe_edge_runtime_message_broker::{QueueError, TaskQueue as _, TaskQueueFactory};

    let queue = TaskQueueFactory::kafka("127.0.0.1:9999", "test-group", "test-topic")
        .expect("client builds");
    let result = queue.health_check().await;
    assert!(
        matches!(result, Err(QueueError::Connection(_))),
        "health_check must return Connection error for unreachable broker, got: {result:?}"
    );
}

/// @covers: TaskQueueFactory::kafka — enqueue fails for an unreachable broker.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_task_queue_enqueue_fails_for_unreachable_broker() {
    use swe_edge_runtime_message_broker::{QueueError, Task, TaskQueue as _, TaskQueueFactory};

    let queue = TaskQueueFactory::kafka("127.0.0.1:9999", "test-group", "test-topic")
        .expect("client builds");
    let result = queue.enqueue(Task::new(b"payload".as_ref())).await;
    assert!(
        matches!(result, Err(QueueError::Enqueue(_))),
        "enqueue must return Enqueue error for unreachable broker, got: {result:?}"
    );
}

/// @covers: TaskQueueFactory::kafka — dequeue returns None when broker is unreachable
/// within the poll timeout.
#[cfg(feature = "kafka")]
#[tokio::test]
async fn test_kafka_task_queue_dequeue_returns_none_when_no_broker() {
    use swe_edge_runtime_message_broker::{TaskQueue as _, TaskQueueFactory};

    let queue = TaskQueueFactory::kafka("127.0.0.1:9999", "test-group", "test-topic")
        .expect("client builds");
    // With no broker reachable, recv() will time out and dequeue returns None.
    let result = queue.dequeue().await;
    // Either None (timeout) or Dequeue error — both are acceptable.
    // The key invariant: dequeue must not hang indefinitely.
    match result {
        Ok(None) => {}
        Ok(Some(_)) => panic!("unexpected task from unreachable broker"),
        Err(_) => {} // connection or dequeue error is also acceptable
    }
}
