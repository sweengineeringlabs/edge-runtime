//! Task queue core layer — shared constants for queue backends.

/// Number of messages to fetch per JetStream pull request.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_FETCH_BATCH_SIZE: usize = 1;

/// Idle heartbeat interval for JetStream consumers in seconds.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_HEARTBEAT_SECS: u64 = 5;
