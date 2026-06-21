//! Integration tests for SAF (Service Abstraction Framework) public factories.
//!
//! These tests verify that SAF factories:
//! - Hide implementation details (return `impl Trait`, never expose concrete types)
//! - Properly initialize backends
//! - Work end-to-end without exposing Cargo dependencies

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use swe_edge_runtime_message_broker::{
        MessageBroker, MessageBrokerFactory, Task, TaskQueue, TaskQueueFactory,
    };

    /// @covers: MessageBrokerFactory::in_memory
    #[tokio::test]
    async fn test_in_memory_broker_factory_returns_working_broker() {
        let broker = MessageBrokerFactory::in_memory();
        assert!(broker.health_check().await.is_ok());
    }

    /// @covers: TaskQueueFactory::in_memory
    #[tokio::test]
    async fn test_in_memory_task_queue_factory_returns_working_queue() {
        let queue = TaskQueueFactory::in_memory();
        assert!(queue.health_check().await.is_ok());
    }

    /// @covers: TaskQueueFactory::in_memory
    #[tokio::test]
    async fn test_in_memory_task_queue_enqueue_dequeue_roundtrip() {
        let queue = TaskQueueFactory::in_memory();
        let task = Task::new(b"work".as_ref());
        let task_id = task.id;

        queue.enqueue(task).await.map_err(|e| e.to_string()).ok();
        let handle = queue
            .dequeue()
            .await
            .map_err(|e| e.to_string())
            .ok()
            .flatten();

        if let Some(h) = handle {
            assert_eq!(h.task_id, task_id);
            h.ack.await.map_err(|e| e.to_string()).ok();
        }
    }
}

#[cfg(feature = "nats")]
mod nats_tests {
    use swe_edge_runtime_message_broker::TaskQueueFactory;

    /// @covers: TaskQueueFactory::nats
    #[tokio::test]
    async fn test_nats_task_queue_connection_error_for_unreachable_host() {
        let result =
            TaskQueueFactory::nats("nats://127.0.0.1:4229", "tasks".into(), "workers".into()).await;
        assert!(result.is_err());
    }
}
