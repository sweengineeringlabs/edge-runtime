//! Integration tests for the NATS task queue API marker.

/// @covers: TaskQueueFactory::nats
#[cfg(feature = "nats")]
#[tokio::test]
async fn test_nats_task_queue_connect_fails_for_unreachable_host() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let result =
        TaskQueueFactory::nats("nats://127.0.0.1:4229", "tasks".into(), "workers".into()).await;
    assert!(result.is_err(), "expected error for unreachable NATS host");
}
