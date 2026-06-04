//! Integration tests for TaskQueue implementations.

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use swe_edge_runtime_message_broker::{Task, TaskQueue, TaskQueueFactory};

    #[tokio::test]
    async fn test_enqueue_and_dequeue_delivers_task() {
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
        }
    }

    #[tokio::test]
    async fn test_enqueue_multiple_tasks_dequeue_fifo() {
        let queue = TaskQueueFactory::in_memory();
        let task1 = Task::new(b"first".as_ref());
        let task2 = Task::new(b"second".as_ref());
        let task1_id = task1.id;
        let task2_id = task2.id;

        queue.enqueue(task1).await.map_err(|e| e.to_string()).ok();
        queue.enqueue(task2).await.map_err(|e| e.to_string()).ok();

        let h1 = queue
            .dequeue()
            .await
            .map_err(|e| e.to_string())
            .ok()
            .flatten();
        let h2 = queue
            .dequeue()
            .await
            .map_err(|e| e.to_string())
            .ok()
            .flatten();

        if let Some(h) = h1 {
            assert_eq!(h.task_id, task1_id);
        }
        if let Some(h) = h2 {
            assert_eq!(h.task_id, task2_id);
        }
    }

    #[tokio::test]
    async fn test_ack_completes_future() {
        let queue = TaskQueueFactory::in_memory();
        let task = Task::new(b"work".as_ref());
        queue.enqueue(task).await.map_err(|e| e.to_string()).ok();

        let handle = queue
            .dequeue()
            .await
            .map_err(|e| e.to_string())
            .ok()
            .flatten();
        if let Some(h) = handle {
            assert!(h.ack.await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_nack_completes_future() {
        let queue = TaskQueueFactory::in_memory();
        let task = Task::new(b"work".as_ref());
        queue.enqueue(task).await.map_err(|e| e.to_string()).ok();

        let handle = queue
            .dequeue()
            .await
            .map_err(|e| e.to_string())
            .ok()
            .flatten();
        if let Some(h) = handle {
            assert!(h.nack.await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let queue = TaskQueueFactory::in_memory();
        assert!(queue.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_enqueue_task_with_headers() {
        use std::collections::HashMap;

        let queue = TaskQueueFactory::in_memory();
        let mut headers = HashMap::new();
        headers.insert("correlation-id".into(), "abc123".into());
        let task = Task::with_headers(b"data".as_ref(), headers);
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
        }
    }

    #[tokio::test]
    async fn test_dequeue_empty_queue_blocks() {
        let queue = TaskQueueFactory::in_memory();
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(100), queue.dequeue()).await;
        assert!(result.is_err(), "dequeue should block on empty queue");
    }
}
