//! API interface surface for the NATS task queue implementation.
//!
//! The implementation is accessed via [`crate::nats_task_queue`].

/// API marker type identifying the NATS JetStream task queue backend.
///
/// Consumers use factory functions from `saf/` — not this type directly.
pub struct NatsTaskQueue;
