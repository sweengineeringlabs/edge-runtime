//! API module for the Kafka task queue backend.
//!
//! The concrete backend lives in `spi/kafka/task/queue/`; consumers obtain an
//! instance via [`crate::TaskQueueFactory::kafka`], which returns `impl TaskQueue`.
