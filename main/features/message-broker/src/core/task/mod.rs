//! Task core layer — shared constants for task queue backends.

pub(crate) mod queue;

/// Visibility timeout for nacked NATS JetStream tasks before redelivery (seconds).
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_VISIBILITY_TIMEOUT_SECS: u64 = 300;

/// Maximum number of pending acks before JetStream applies backpressure.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_MAX_ACK_PENDING: i64 = 1000;

/// Default in-memory task queue channel capacity.
///
/// Sized to buffer a typical batch of work without excessive memory usage.
/// Producers that exceed this limit will apply backpressure to the caller.
pub(crate) const DEFAULT_TASK_QUEUE_CAPACITY: usize = 1024;

/// Maximum task payload size for in-memory queues (4 MiB).
///
/// Payloads exceeding this limit are rejected at enqueue time to avoid
/// unbounded memory consumption inside the channel buffer.
#[expect(
    dead_code,
    reason = "used in tokio-rt feature — dead when tokio-rt is disabled"
)]
pub(crate) const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
