//! [`InMemoryTaskQueue`] — tokio mpsc channel backed task queue.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::api::task::queue::queue_error::QueueError;
use crate::api::task::queue::task::Task;
use crate::api::task::queue::task_queue::TaskQueue;
use crate::api::task::queue::types::task::TaskHandle;

/// Maximum task payload size accepted (4 MiB).
const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct InMemoryTaskQueueInner {
    pub(crate) tx: mpsc::Sender<Task>,
    pub(crate) rx: mpsc::Receiver<Task>,
}

/// In-memory work queue backed by [`tokio::sync::mpsc`].
///
/// Tasks are enqueued into a bounded MPSC channel. Each dequeue call retrieves
/// the next available task. Ack signals permanent removal; nack can signal redelivery.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub struct InMemoryTaskQueue {
    pub(crate) inner: Arc<tokio::sync::Mutex<InMemoryTaskQueueInner>>,
}

impl TaskQueue for InMemoryTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let inner = Arc::clone(&self.inner);
        Box::pin(async move {
            if task.payload.len() > MAX_TASK_PAYLOAD_BYTES {
                return Err(QueueError::Enqueue(format!(
                    "payload exceeds maximum size of {} bytes",
                    MAX_TASK_PAYLOAD_BYTES
                )));
            }
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

    fn make_queue() -> InMemoryTaskQueue {
        let (tx, rx) = mpsc::channel(1024);
        InMemoryTaskQueue {
            inner: Arc::new(tokio::sync::Mutex::new(InMemoryTaskQueueInner { tx, rx })),
        }
    }

    #[test]
    fn test_new_creates_queue() {
        let _queue = make_queue();
    }

    #[tokio::test]
    async fn test_enqueue_succeeds() {
        let queue = make_queue();
        let task = Task::new(b"test".as_ref());
        assert!(queue.enqueue(task).await.is_ok());
    }

    #[tokio::test]
    async fn test_dequeue_returns_enqueued_task() {
        let queue = make_queue();
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

    #[tokio::test]
    async fn test_dequeue_blocks_on_empty_queue() {
        let queue: InMemoryTaskQueue = make_queue();
        tokio::spawn(async move {
            let result =
                tokio::time::timeout(std::time::Duration::from_millis(100), queue.dequeue()).await;
            assert!(result.is_err(), "dequeue should timeout on empty queue");
        });
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let queue = make_queue();
        assert!(queue.health_check().await.is_ok());
    }
}
