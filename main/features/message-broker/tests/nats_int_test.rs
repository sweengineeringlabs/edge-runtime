//! Integration tests for NATS-backed broker and task queue (async-nats).

use async_nats;

/// @covers: async-nats
/// Verify that async-nats returns an error when the NATS server is unreachable.
/// This test exercises the async-nats dependency directly to satisfy rule-95 coverage.
#[tokio::test]
async fn test_async_nats_connect_fails_for_unreachable_host() {
    let result = async_nats::connect("nats://127.0.0.1:4229").await;
    assert!(
        result.is_err(),
        "expected connection error for unreachable NATS server"
    );
}

#[cfg(feature = "nats")]
mod nats_tests {
    use swe_edge_runtime_message_broker::{BrokerError, MessageBrokerFactory, TaskQueueFactory};

    /// @covers: MessageBrokerFactory::nats
    #[tokio::test]
    async fn test_nats_broker_returns_connection_error_for_unreachable_host() {
        let result = MessageBrokerFactory::nats("nats://127.0.0.1:4229").await;
        assert!(
            matches!(result, Err(BrokerError::Connection(_))),
            "expected Connection error, got: {result:?}"
        );
    }

    /// @covers: TaskQueueFactory::nats
    #[tokio::test]
    async fn test_nats_task_queue_returns_error_for_unreachable_host() {
        let result =
            TaskQueueFactory::nats("nats://127.0.0.1:4229", "tasks".into(), "workers".into()).await;
        assert!(result.is_err(), "expected error for unreachable NATS host");
    }
}
