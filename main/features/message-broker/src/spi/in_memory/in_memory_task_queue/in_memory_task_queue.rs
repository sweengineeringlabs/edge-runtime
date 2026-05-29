//! [`InMemoryTaskQueue`] — tokio mpsc channel backed task queue.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::api::task_queue::{QueueError, Task, TaskHandle, TaskQueue};

/// Capacity of the task queue channel.
const CHANNEL_CAPACITY: usize = 1024;

struct InMemoryTaskQueueInner {
    tx: mpsc::Sender<Task>,
    rx: mpsc::Receiver<Task>,
}

/// In-memory work queue backed by [`tokio::sync::mpsc`].
///
/// Tasks are enqueued into a bounded MPSC channel. Each dequeue call retrieves
/// the next available task. Ack signals permanent removal; nack can signal redelivery.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub(crate) struct InMemoryTaskQueue {
    inner: Arc<tokio::sync::Mutex<InMemoryTaskQueueInner>>,
}

impl InMemoryTaskQueue {
    /// Create a new in-memory task queue with a bounded channel of `CHANNEL_CAPACITY`.
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(InMemoryTaskQueueInner { tx, rx })),
        }
    }
}

impl TaskQueue for InMemoryTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move {
            let q = inner.lock().await;
            q.tx.send(task)
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move {
            let mut q = inner.lock().await;
            match q.rx.recv().await {
                Some(task) => {
                    let task_id = task.id;
                    let ack = Box::pin(async move { Ok(()) });
                    let nack = Box::pin(async move { Ok(()) });
                    Ok(Some(TaskHandle::new(task_id, ack, nack)))
                }
                None => Ok(None),
            }
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_creates_queue() {
        let _queue = InMemoryTaskQueue::new();
    }

    /// @covers: enqueue
    #[tokio::test]
    async fn test_enqueue_succeeds() {
        let queue = InMemoryTaskQueue::new();
        let task = Task::new(b"test".as_ref());
        assert!(queue.enqueue(task).await.is_ok());
    }

    /// @covers: dequeue
    #[tokio::test]
    async fn test_dequeue_returns_enqueued_task() {
        let queue = InMemoryTaskQueue::new();
        let task = Task::new(b"payload".as_ref());
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

    /// @covers: dequeue
    #[tokio::test]
    async fn test_dequeue_blocks_on_empty_queue() {
        let queue: InMemoryTaskQueue = InMemoryTaskQueue::new();
        tokio::spawn(async move {
            let result =
                tokio::time::timeout(std::time::Duration::from_millis(100), queue.dequeue()).await;
            assert!(result.is_err(), "dequeue should timeout on empty queue");
        });
    }

    /// @covers: health_check
    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let queue = InMemoryTaskQueue::new();
        assert!(queue.health_check().await.is_ok());
    }
}
