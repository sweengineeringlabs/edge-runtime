//! API module for the NATS broker backend.
//!
//! The concrete backend lives in `spi/nats/message/broker/`; consumers obtain
//! an instance via [`crate::MessageBrokerFactory::nats`], which returns
//! `impl MessageBroker`.
