//! Broker core layer — shared constraints and capacity constants.

/// Maximum byte length of a topic name (inclusive).
///
/// Topics exceeding this limit are rejected at publish and subscribe time to
/// prevent unbounded memory growth in the channel map key space.
pub(crate) const MAX_TOPIC_BYTES: usize = 256;

/// Default broadcast channel capacity per topic.
///
/// When a topic's sender is created, this many messages can be buffered before
/// slow receivers start lagging. If a receiver falls more than this many messages
/// behind, it receives a `StreamLagged` error on the next recv.
pub(crate) const DEFAULT_CHANNEL_CAPACITY: usize = 1024;

/// Maximum number of concurrent subscriptions per topic.
///
/// Attempting to subscribe beyond this limit returns a capacity error.
/// Sized to accommodate typical microservice fan-out without unbounded growth.
#[expect(
    dead_code,
    reason = "reserved for capacity enforcement — dead until subscriber limit is enforced"
)]
pub(crate) const MAX_SUBSCRIPTIONS_PER_TOPIC: usize = 64;

/// Minimum non-empty topic name length in bytes.
#[expect(
    dead_code,
    reason = "reserved for stricter topic validation — dead until validation is enforced"
)]
pub(crate) const MIN_TOPIC_BYTES: usize = 1;
