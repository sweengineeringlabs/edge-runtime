//! Task core layer — shared constants for task queue backends.

pub(crate) mod queue;

/// Maximum task payload size accepted by all backends (4 MiB).
pub(crate) const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;

/// Maximum number of task headers per enqueue call.
pub(crate) const MAX_TASK_HEADERS: usize = 32;

/// Visibility timeout for nacked NATS JetStream tasks before redelivery (seconds).
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_VISIBILITY_TIMEOUT_SECS: u64 = 300;

/// Maximum number of pending acks before JetStream applies backpressure.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_MAX_ACK_PENDING: i64 = 1000;
