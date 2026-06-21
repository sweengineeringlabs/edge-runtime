//! Threshold policy API surface — constants used by the core scaling policy.

/// Default maximum concurrent in-flight requests before triggering scale-out.
pub const DEFAULT_REQUESTS_ACTIVE_MAX: u64 = 500;
/// Default maximum requests per second before triggering scale-out.
pub const DEFAULT_REQUESTS_PER_SEC_MAX: u64 = 1_000;
/// Default maximum p99 latency in milliseconds before triggering scale-out.
pub const DEFAULT_LATENCY_P99_MS_MAX: f64 = 200.0;
