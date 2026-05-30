//! Task core layer — shared constants for task queue backends.

pub(crate) mod queue;

/// Visibility timeout for nacked NATS JetStream tasks before redelivery (seconds).
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_VISIBILITY_TIMEOUT_SECS: u64 = 300;

/// Maximum number of pending acks before JetStream applies backpressure.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_MAX_ACK_PENDING: i64 = 1000;
