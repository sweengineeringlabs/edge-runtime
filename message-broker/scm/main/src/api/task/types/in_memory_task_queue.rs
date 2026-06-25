//! [`InMemoryTaskQueue`] — tokio mpsc channel backed task queue.

use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::api::task::types::task::Task;

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
