//! NATS task queue backend (JetStream-backed).

#[allow(clippy::module_inception)]
pub(crate) mod nats_task_queue;

pub(crate) use nats_task_queue::NatsTaskQueue;
