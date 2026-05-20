//! Integration tests for SAF (Service Abstraction Framework) public factories.
//!
//! These tests verify that SAF factories:
//! - Hide implementation details (return `impl Trait`, never expose concrete types)
//! - Properly initialize backends
//! - Work end-to-end without exposing Cargo dependencies

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use swe_edge_runtime_message_broker::{
        in_memory_broker, in_memory_task_queue, MessageBroker, Task, TaskQueue,
    };

    /// @covers: in_memory_broker
    #[tokio::test]
    async fn test_in_memory_broker_factory_returns_working_broker() {
        let broker = in_memory_broker();
        assert!(broker.health_check().await.is_ok());
    }

    /// @covers: in_memory_task_queue
    #[tokio::test]
    async fn test_in_memory_task_queue_factory_returns_working_queue() {
        let queue = in_memory_task_queue();
        assert!(queue.health_check().await.is_ok());
    }

    /// @covers: in_memory_task_queue
    #[tokio::test]
    async fn test_in_memory_task_queue_enqueue_dequeue_roundtrip() {
        let queue = in_memory_task_queue();
        let task = Task::new(b"work".as_ref());
        let task_id = task.id;

        queue.enqueue(task).await.unwrap();
        let handle = queue
            .dequeue()
            .await
            .unwrap()
            .expect("should have dequeued task");

        assert_eq!(handle.task_id, task_id);
        assert!(handle.ack.await.is_ok());
    }
}

#[cfg(feature = "nats")]
mod nats_tests {
    use swe_edge_runtime_message_broker::nats_task_queue;

    /// @covers: nats_task_queue
    #[tokio::test]
    async fn test_nats_task_queue_connection_error_for_unreachable_host() {
        // Factory should hide async_nats::jetstream::Context completely
        // Verify it takes only generic string parameters
        let result =
            nats_task_queue("nats://127.0.0.1:4229", "tasks".into(), "workers".into()).await;
        // Should fail with QueueError::Connection, not expose implementation types
        assert!(result.is_err());
    }
}
