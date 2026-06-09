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

// ── Live-broker tests ────────────────────────────────────────────────────────
//
// Run with a real Kafka cluster:
//
//   KAFKA_BROKERS=localhost:9092 cargo test --features kafka -- \
//     --include-ignored --test kafka_task_queue_int_test
//
// All tests below are ignored unless explicitly included so normal CI (without
// a Kafka sidecar) still passes green.

/// Returns the broker address from KAFKA_BROKERS, or panics with a clear message.
#[cfg(feature = "kafka")]
fn require_kafka_brokers() -> String {
    std::env::var("KAFKA_BROKERS")
        .expect("KAFKA_BROKERS env var must be set to run live-broker tests (e.g. localhost:9092)")
}

/// @covers: enqueue + dequeue + ack — happy-path roundtrip with a live broker.
///
/// Verifies that a payload written by `enqueue` is returned verbatim by the
/// first `dequeue` call and that `ack` commits without error.
#[cfg(feature = "kafka")]
#[tokio::test]
#[ignore = "requires-kafka"]
async fn test_enqueue_dequeue_ack_roundtrip_with_live_broker() {
    use bytes::Bytes;
    use swe_edge_runtime_message_broker::{Task, TaskQueue as _, TaskQueueFactory};

    let brokers = require_kafka_brokers();
    let topic = "swe-edge-test-enqueue-dequeue-ack";
    let payload = b"roundtrip-payload";

    let queue = TaskQueueFactory::kafka(&brokers, "swe-edge-test-group-ack", topic)
        .expect("queue construction must succeed with a live broker");

    queue
        .enqueue(Task::new(payload.as_ref()))
        .await
        .expect("enqueue must succeed with a live broker");

    // Retry dequeue briefly — partition assignment can lag after subscribe.
    let handle = {
        let mut handle = None;
        for _ in 0..30 {
            match queue.dequeue().await.expect("dequeue must not error") {
                Some(h) => {
                    handle = Some(h);
                    break;
                }
                None => tokio::time::sleep(std::time::Duration::from_millis(200)).await,
            }
        }
        handle.expect("dequeue must return the enqueued task within 6 s")
    };

    assert_eq!(
        handle.task.payload,
        Bytes::from_static(payload),
        "dequeued payload must match the enqueued payload"
    );

    handle
        .ack
        .await
        .expect("ack must succeed with a live broker");
}

/// @covers: enqueue + dequeue + nack — nack redelivers the task to the same consumer.
///
/// Verifies that after `nack` the same payload is returned by the next `dequeue`
/// call without re-publishing.
#[cfg(feature = "kafka")]
#[tokio::test]
#[ignore = "requires-kafka"]
async fn test_enqueue_dequeue_nack_redelivers_with_live_broker() {
    use bytes::Bytes;
    use swe_edge_runtime_message_broker::{Task, TaskQueue as _, TaskQueueFactory};

    let brokers = require_kafka_brokers();
    let topic = "swe-edge-test-enqueue-dequeue-nack";
    let payload = b"nack-payload";

    let queue = TaskQueueFactory::kafka(&brokers, "swe-edge-test-group-nack", topic)
        .expect("queue construction must succeed with a live broker");

    queue
        .enqueue(Task::new(payload.as_ref()))
        .await
        .expect("enqueue must succeed with a live broker");

    // First dequeue — get the message.
    let first = {
        let mut handle = None;
        for _ in 0..30 {
            match queue.dequeue().await.expect("dequeue must not error") {
                Some(h) => {
                    handle = Some(h);
                    break;
                }
                None => tokio::time::sleep(std::time::Duration::from_millis(200)).await,
            }
        }
        handle.expect("first dequeue must return the enqueued task within 6 s")
    };

    assert_eq!(
        first.task.payload,
        Bytes::from_static(payload),
        "first dequeue must return the published payload"
    );

    // Nack — rdkafka seeks back to the same offset.
    first
        .nack
        .await
        .expect("nack must succeed with a live broker");

    // Second dequeue — must redeliver the same message.
    let second = {
        let mut handle = None;
        for _ in 0..30 {
            match queue.dequeue().await.expect("dequeue must not error") {
                Some(h) => {
                    handle = Some(h);
                    break;
                }
                None => tokio::time::sleep(std::time::Duration::from_millis(200)).await,
            }
        }
        handle.expect("second dequeue must redeliver the nacked task within 6 s")
    };

    assert_eq!(
        second.task.payload,
        Bytes::from_static(payload),
        "nack must cause the same payload to be redelivered"
    );

    // Clean up: ack the redelivered message so it doesn't pollute other test runs.
    second.ack.await.expect("final ack must succeed");
}

/// @covers: LoggingConsumerContext — rebalance events are observable via tracing.
///
/// This test verifies that construction with the logging context succeeds and
/// that the queue operates normally end-to-end when the context is wired in.
/// Rebalance events are emitted via `tracing::info!`; capturing the tracing
/// output requires a subscriber such as `tracing-subscriber` with a
/// `TestWriter` — left as a manual verification step in the live environment.
#[cfg(feature = "kafka")]
#[tokio::test]
#[ignore = "requires-kafka"]
async fn test_kafka_task_queue_logging_consumer_context_wired_with_live_broker() {
    use swe_edge_runtime_message_broker::{Task, TaskQueue as _, TaskQueueFactory};

    let brokers = require_kafka_brokers();
    let topic = "swe-edge-test-logging-context";

    let queue = TaskQueueFactory::kafka(&brokers, "swe-edge-test-group-logging", topic)
        .expect("queue with LoggingConsumerContext must construct successfully");

    // Exercise both directions to confirm the wired LoggingConsumerContext doesn't break IO.
    queue
        .enqueue(Task::new(b"context-test".as_ref()))
        .await
        .expect("enqueue must succeed with LoggingConsumerContext wired");

    let handle = {
        let mut handle = None;
        for _ in 0..30 {
            match queue.dequeue().await.expect("dequeue must not error") {
                Some(h) => {
                    handle = Some(h);
                    break;
                }
                None => tokio::time::sleep(std::time::Duration::from_millis(200)).await,
            }
        }
        handle.expect("dequeue must succeed with LoggingConsumerContext wired")
    };

    handle
        .ack
        .await
        .expect("ack must succeed with LoggingConsumerContext wired");
}
