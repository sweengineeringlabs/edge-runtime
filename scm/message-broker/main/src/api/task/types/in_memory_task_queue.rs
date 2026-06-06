//! [`InMemoryTaskQueue`] — tokio mpsc channel backed task queue.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::{mpsc, Mutex};

use crate::api::task::error::queue_error::QueueError;
use crate::api::task::traits::task_queue::TaskQueue;
use crate::api::task::types::task::Task;
use crate::api::task::types::task_handle::TaskHandle;

/// Maximum task payload size accepted (4 MiB).
const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;

/// In-memory work queue backed by [`tokio::sync::mpsc`].
///
/// Tasks are enqueued into a bounded MPSC channel. Each dequeue call retrieves
/// the next available task. Ack signals permanent removal; nack can signal redelivery.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub struct InMemoryTaskQueue {
    pub(crate) tx: Arc<mpsc::Sender<Task>>,
    pub(crate) rx: Arc<Mutex<mpsc::Receiver<Task>>>,
}

impl TaskQueue for InMemoryTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            if task.payload.len() > MAX_TASK_PAYLOAD_BYTES {
                return Err(QueueError::Enqueue(format!(
                    "payload exceeds maximum size of {} bytes",
                    MAX_TASK_PAYLOAD_BYTES
                )));
            }
            tx.send(task)
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        let rx = Arc::clone(&self.rx);
        Box::pin(async move {
            let mut guard = rx.lock().await;
            match guard.recv().await {
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
