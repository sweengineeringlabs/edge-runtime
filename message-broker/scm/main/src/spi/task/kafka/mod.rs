//! Kafka task queue backend (rdkafka-backed competing consumer).

#[allow(clippy::module_inception)]
pub(crate) mod kafka_task_queue;
pub(crate) mod logging_consumer_context;

pub(crate) use kafka_task_queue::KafkaTaskQueue;
