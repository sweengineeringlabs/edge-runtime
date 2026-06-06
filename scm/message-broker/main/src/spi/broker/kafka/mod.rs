//! Kafka message broker backend (rdkafka-backed).

#[allow(clippy::module_inception)]
pub(crate) mod kafka_message_broker;

pub(crate) use kafka_message_broker::KafkaMessageBroker;
