//! API interface surface for the in-memory task queue implementation.
//!
//! The implementation is accessed via [`crate::in_memory_task_queue`].

/// API marker type identifying the in-memory task queue backend.
///
/// Consumers use factory functions from `saf/` — not this type directly.
pub struct InMemoryTaskQueue;
