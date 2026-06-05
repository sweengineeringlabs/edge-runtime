//! API module for the Kafka broker backend.
//!
//! The concrete backend lives in `spi/kafka/message/broker/`; consumers obtain
//! an instance via [`crate::MessageBrokerFactory::kafka`], which returns
//! `impl MessageBroker`.
