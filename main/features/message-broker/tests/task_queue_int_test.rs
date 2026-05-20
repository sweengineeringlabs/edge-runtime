//! Integration tests for TaskQueue implementations.

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use swe_edge_runtime_message_broker::{in_memory_task_queue, Task, TaskQueue};

    #[tokio::test]
    async fn test_enqueue_and_dequeue_delivers_task() {
        let queue = in_memory_task_queue();
        let task = Task::new(b"work".as_ref());
        let task_id = task.id;

        queue.enqueue(task).await.unwrap();

        let handle = queue
            .dequeue()
            .await
            .expect("dequeue failed")
            .expect("no task in queue");
        assert_eq!(handle.task_id, task_id);
    }

    #[tokio::test]
    async fn test_enqueue_multiple_tasks_dequeue_fifo() {
        let queue = in_memory_task_queue();
        let task1 = Task::new(b"first".as_ref());
        let task2 = Task::new(b"second".as_ref());
        let task1_id = task1.id;
        let task2_id = task2.id;

        queue.enqueue(task1).await.unwrap();
        queue.enqueue(task2).await.unwrap();

        let handle1 = queue
            .dequeue()
            .await
            .expect("dequeue failed")
            .expect("no first task");
        assert_eq!(handle1.task_id, task1_id);

        let handle2 = queue
            .dequeue()
            .await
            .expect("dequeue failed")
            .expect("no second task");
        assert_eq!(handle2.task_id, task2_id);
    }

    #[tokio::test]
    async fn test_ack_completes_future() {
        let queue = in_memory_task_queue();
        let task = Task::new(b"work".as_ref());
        queue.enqueue(task).await.unwrap();

        let handle = queue.dequeue().await.unwrap().expect("no task in queue");
        let result = handle.ack.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_nack_completes_future() {
        let queue = in_memory_task_queue();
        let task = Task::new(b"work".as_ref());
        queue.enqueue(task).await.unwrap();

        let handle = queue.dequeue().await.unwrap().expect("no task in queue");
        let result = handle.nack.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let queue = in_memory_task_queue();
        assert!(queue.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_enqueue_task_with_headers() {
        use std::collections::HashMap;

        let queue = in_memory_task_queue();
        let mut headers = HashMap::new();
        headers.insert("correlation-id".into(), "abc123".into());
        let task = Task::with_headers(b"data".as_ref(), headers);
        let task_id = task.id;

        queue.enqueue(task).await.unwrap();

        let handle = queue.dequeue().await.unwrap().expect("no task in queue");
        assert_eq!(handle.task_id, task_id);
    }

    #[tokio::test]
    async fn test_dequeue_empty_queue_blocks() {
        let queue = in_memory_task_queue();

        let result =
            tokio::time::timeout(std::time::Duration::from_millis(100), queue.dequeue()).await;

        // Should timeout waiting for a task
        assert!(result.is_err(), "dequeue should block on empty queue");
    }
}
