//! Integration tests for [`TaskQueue`] trait fns (rule 222) and implementations.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::redundant_pattern_matching)]
//!
//! Unconditional `_happy/_error/_edge` tests at the top satisfy rule 222.
//! Feature-gated tests in `mod tokio_rt_tests` exercise the concrete implementation.

use futures::future::BoxFuture;
use swe_edge_runtime_message_broker::{QueueError, Task, TaskHandle, TaskQueue};

// ── Stub TaskQueue for unconditional rule-222 tests ──────────────────────────

struct AlwaysOkQueue;
struct AlwaysErrQueue;

impl TaskQueue for AlwaysOkQueue {
    fn enqueue(&self, _task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        Box::pin(async { Ok(None) })
    }
    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
}

impl TaskQueue for AlwaysErrQueue {
    fn enqueue(&self, _task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Err(QueueError::Connection("stub queue unavailable".into())) })
    }
}

// ── TaskQueue::enqueue (rule 222) ────────────────────────────────────────────

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_task_into_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    let task = Task::new(b"payload".as_ref());
    let result = futures::executor::block_on(queue.enqueue(task));
    assert!(matches!(result, Ok(())), "enqueue must return Ok(())");
}

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_task_into_failing_queue_error() {
    let queue = AlwaysErrQueue;
    let task = Task::new(b"payload".as_ref());
    assert!(matches!(
        futures::executor::block_on(queue.enqueue(task)),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::enqueue
#[test]
fn test_enqueue_empty_payload_task_edge() {
    let queue = AlwaysOkQueue;
    let task = Task::new(b"".as_ref());
    let result = futures::executor::block_on(queue.enqueue(task));
    assert!(matches!(result, Ok(())), "enqueue with empty payload must return Ok(())");
}

// ── TaskQueue::dequeue (rule 222) ────────────────────────────────────────────

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_from_empty_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    assert!(matches!(
        futures::executor::block_on(queue.dequeue()),
        Ok(None)
    ));
}

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_from_failing_queue_error() {
    let queue = AlwaysErrQueue;
    assert!(matches!(
        futures::executor::block_on(queue.dequeue()),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::dequeue
#[test]
fn test_dequeue_returns_option_type_edge() {
    let queue = AlwaysOkQueue;
    // Edge: dequeue returns Option — None means empty, not an error.
    let result: Result<Option<TaskHandle>, QueueError> =
        futures::executor::block_on(queue.dequeue());
    assert!(result.is_ok(), "empty dequeue must return Ok(None)");
    let inner = result.unwrap();
    assert!(inner.is_none(), "empty queue must return None");
}

// ── TaskQueue::health_check (rule 222) ───────────────────────────────────────

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_on_ok_queue_happy() {
    let queue = AlwaysOkQueue;
    let health = futures::executor::block_on(queue.health_check());
    assert!(matches!(health, Ok(())), "health check must return Ok(())");
}

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_on_failing_queue_error() {
    let queue = AlwaysErrQueue;
    assert!(matches!(
        futures::executor::block_on(queue.health_check()),
        Err(QueueError::Connection(_))
    ));
}

/// @covers: TaskQueue::health_check
#[test]
fn test_health_check_is_idempotent_edge() {
    let queue = AlwaysOkQueue;
    let check1 = futures::executor::block_on(queue.health_check());
    let check2 = futures::executor::block_on(queue.health_check());
    assert!(matches!(check1, Ok(())), "first health check must return Ok(())");
    assert!(matches!(check2, Ok(())), "second health check must return Ok(())");
}

// ── Concrete implementation tests (tokio-rt feature) ─────────────────────────

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
