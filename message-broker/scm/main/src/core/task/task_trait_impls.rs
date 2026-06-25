//! Trait implementations for task api types.

use crate::api::TaskId;
use crate::api::TaskQueueFactory;
use crate::api::TaskQueueFactoryContract;
#[cfg(feature = "tokio-rt")]
use crate::api::InMemoryTaskQueue;
#[cfg(feature = "tokio-rt")]
use crate::api::QueueError;
#[cfg(feature = "tokio-rt")]
use crate::api::Task;
#[cfg(feature = "tokio-rt")]
use crate::api::TaskHandle;
#[cfg(feature = "tokio-rt")]
use crate::api::TaskQueue;
#[cfg(feature = "tokio-rt")]
use futures::future::BoxFuture;

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TaskQueueFactoryContract for TaskQueueFactory {
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(&self) -> InMemoryTaskQueue {
        TaskQueueFactory::in_memory()
    }
}

#[cfg(feature = "tokio-rt")]
impl TaskQueue for InMemoryTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        use std::sync::Arc;
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            if task.payload.len() > super::MAX_TASK_PAYLOAD_BYTES {
                return Err(QueueError::Enqueue(format!(
                    "payload exceeds maximum size of {} bytes",
                    super::MAX_TASK_PAYLOAD_BYTES
                )));
            }
            tx.send(task)
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        use std::sync::Arc;
        let rx = Arc::clone(&self.rx);
        Box::pin(async move {
            let mut guard = rx.lock().await;
            match guard.recv().await {
                Some(task) => {
                    let ack: BoxFuture<'static, Result<(), QueueError>> =
                        Box::pin(async { Ok(()) });
                    let nack: BoxFuture<'static, Result<(), QueueError>> =
                        Box::pin(async { Ok(()) });
                    Ok(Some(TaskHandle::new(
                        task.id,
                        task.payload,
                        task.headers,
                        ack,
                        nack,
                    )))
                }
                None => Ok(None),
            }
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        Box::pin(async { Ok(()) })
    }
}
