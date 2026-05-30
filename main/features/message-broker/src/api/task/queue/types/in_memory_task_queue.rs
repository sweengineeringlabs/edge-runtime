//! API marker type for the in-memory task queue implementation.

/// API marker type identifying the in-memory task queue backend.
///
/// Consumers use factory functions from `saf/` — not this type directly.
pub struct InMemoryTaskQueue;
