//! Task queue core layer — shared constants for queue backends.

/// Idle heartbeat interval for JetStream consumers in seconds.
#[cfg(feature = "nats")]
pub(crate) const DEFAULT_HEARTBEAT_SECS: u64 = 5;
