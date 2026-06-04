//! API marker type for the NATS task queue implementation.

/// API marker type identifying the NATS JetStream task queue backend.
///
/// Consumers use factory functions from `saf/` — not this type directly.
pub struct NatsTaskQueue;
