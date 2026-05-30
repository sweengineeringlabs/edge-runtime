//! Core layer — concrete implementations and shared constants for broker backends.

pub(crate) mod broker;
pub(crate) mod task;

/// Default channel capacity for in-memory broker and queue backends.
pub(crate) const DEFAULT_CHANNEL_CAPACITY: usize = 1024;

/// Maximum message payload size across all backends (16 MiB).
pub(crate) const MAX_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;
