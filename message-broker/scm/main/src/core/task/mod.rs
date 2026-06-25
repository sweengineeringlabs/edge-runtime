//! Task core layer — shared constants for task queue backends.

pub(crate) mod queue;
pub(crate) mod task_trait_impls;

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

/// Kafka dequeue poll timeout in milliseconds.
///
/// `dequeue()` waits at most this long for a message before returning `None`.
/// Sized to keep queue workers responsive without spinning.
#[cfg(feature = "kafka")]
pub(crate) const KAFKA_DEQUEUE_POLL_TIMEOUT_MS: u64 = 100;

/// Kafka producer `message.timeout.ms` — maximum time to wait for delivery acknowledgement.
#[cfg(feature = "kafka")]
pub(crate) const KAFKA_MESSAGE_TIMEOUT_MS: &str = "5000";

/// Kafka consumer `session.timeout.ms` — broker considers consumer dead after this interval.
#[cfg(feature = "kafka")]
pub(crate) const KAFKA_SESSION_TIMEOUT_MS: &str = "6000";

/// Kafka health-check metadata fetch timeout in seconds.
#[cfg(feature = "kafka")]
pub(crate) const KAFKA_HEALTH_CHECK_TIMEOUT_SECS: u64 = 5;

/// Kafka subscribe channel capacity — bounds the in-memory buffer between the
/// rdkafka poll loop and the returned [`MessageStream`](crate::api::broker::MessageStream).
///
/// When the channel is full the poll loop yield-waits on `send`, slowing Kafka
/// consumption and applying natural backpressure to slow subscribers.
#[cfg(feature = "kafka")]
pub(crate) const KAFKA_SUBSCRIBE_CHANNEL_CAPACITY: usize = 1024;

/// Maximum task payload size for in-memory queues (4 MiB).
///
/// Payloads exceeding this limit are rejected at enqueue time to avoid
/// unbounded memory consumption inside the channel buffer.
#[cfg(feature = "tokio-rt")]
pub(crate) const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
