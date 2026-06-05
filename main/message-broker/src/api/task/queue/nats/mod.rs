//! API module for the NATS task queue backend.
//!
//! The concrete backend lives in `spi/nats/task/queue/`; consumers obtain an
//! instance via [`crate::TaskQueueFactory::nats`], which returns `impl TaskQueue`.
