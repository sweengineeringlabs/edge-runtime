//! Load monitor — type declarations.
//!
//! `LoadCounters` wraps a `swe_justobserv_metrics::MetricsProvider` and
//! adds the few per-tick atomics needed to compute derived rates (RPS, p99).
//! The provider owns all metric storage and Prometheus-compatible export.

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use swe_observ_metrics::MetricsProvider;

const RING_CAPACITY: usize = 1024;

/// Shared load state — wraps a `MetricsProvider` for durable metric storage
/// and a ring buffer for accurate p99 latency computation.
pub struct LoadCounters {
    pub(crate) provider:             Arc<dyn MetricsProvider>,
    /// Signed so concurrent add/sub never underflows to u64::MAX.
    pub(crate) requests_in_flight:   AtomicI64,
    /// Reset to 0 each tick by the background sampler.
    pub(crate) requests_since_tick:  AtomicU64,
    pub(crate) errors_since_tick:    AtomicU64,
    /// Ring buffer of request latencies in microseconds.
    pub(crate) latency_ring:         Mutex<RingBuffer>,
}

impl LoadCounters {
    /// Construct with the supplied metrics provider.
    pub fn new(provider: Arc<dyn MetricsProvider>) -> Self {
        Self {
            provider,
            requests_in_flight:  AtomicI64::new(0),
            requests_since_tick: AtomicU64::new(0),
            errors_since_tick:   AtomicU64::new(0),
            latency_ring:        Mutex::new(RingBuffer::new(RING_CAPACITY)),
        }
    }

    /// Called at the start of each request.
    pub(crate) fn on_start(&self) {
        self.requests_in_flight.fetch_add(1, Ordering::Relaxed);
    }

    /// Called at the end of each request with measured latency and outcome.
    pub(crate) fn on_end(&self, latency_us: u64, is_error: bool) {
        self.requests_in_flight.fetch_sub(1, Ordering::Relaxed);
        self.requests_since_tick.fetch_add(1, Ordering::Relaxed);
        self.provider.record_counter("edge_requests_total", 1.0, &[]);
        if is_error {
            self.errors_since_tick.fetch_add(1, Ordering::Relaxed);
            self.provider.record_counter("edge_errors_total", 1.0, &[]);
        }
        self.latency_ring.lock().push(latency_us);
    }
}

/// Ring buffer of latency samples in microseconds.
pub(crate) struct RingBuffer {
    buf:  Vec<u64>,
    head: usize,
}

impl RingBuffer {
    pub(crate) fn new(capacity: usize) -> Self {
        Self { buf: vec![0; capacity], head: 0 }
    }

    pub(crate) fn push(&mut self, val_us: u64) {
        let cap = self.buf.len();
        self.buf[self.head % cap] = val_us;
        self.head = self.head.wrapping_add(1);
    }

    /// 99th-percentile latency in milliseconds from the current window.
    pub(crate) fn p99_ms(&self) -> f64 {
        let mut samples: Vec<u64> = self.buf.iter().copied().filter(|&v| v > 0).collect();
        if samples.is_empty() { return 0.0; }
        samples.sort_unstable();
        let idx = (samples.len() * 99 / 100).saturating_sub(1);
        samples[idx] as f64 / 1_000.0 // µs → ms
    }
}

/// Shared handle passed between the monitor wrappers and the metrics server.
pub type SharedCounters = Arc<LoadCounters>;

/// Configuration for the Prometheus metrics endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetricsConfig {
    /// Bind address for the metrics server (separate from `http_bind`).
    pub bind: String,
    /// Path served by the metrics endpoint.
    pub path: String,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:9090".into(),
            path: "/metrics".into(),
        }
    }
}

/// Thresholds that trigger a scale-out signal when exceeded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AutoscalePolicy {
    /// Scale out when in-flight requests exceed this value.
    pub requests_active_max:  u64,
    /// Scale out when requests-per-second exceeds this value.
    pub requests_per_sec_max: u64,
    /// Scale out when p99 latency (ms) exceeds this value.
    pub latency_p99_ms_max:   f64,
}

impl Default for AutoscalePolicy {
    fn default() -> Self {
        Self {
            requests_active_max:  500,
            requests_per_sec_max: 1_000,
            latency_p99_ms_max:   200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use swe_observ_metrics::create_local_metrics_backend;

    fn counters() -> SharedCounters {
        Arc::new(LoadCounters::new(Arc::new(create_local_metrics_backend())))
    }

    /// @covers: LoadCounters::new
    #[test]
    fn test_new_counters_start_at_zero() {
        let c = counters();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_since_tick.load(Ordering::Relaxed), 0);
    }

    /// @covers: LoadCounters::on_start
    #[test]
    fn test_on_start_increments_in_flight() {
        let c = counters();
        c.on_start();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 1);
    }

    /// @covers: LoadCounters::on_end
    #[test]
    fn test_on_end_decrements_in_flight_and_records_total() {
        let c = counters();
        c.on_start();
        c.on_end(500, false);
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_since_tick.load(Ordering::Relaxed), 1);
        let snaps = c.provider.export();
        assert!(snaps.iter().any(|s| s.name == "edge_requests_total" && s.value == 1.0));
    }

    /// @covers: LoadCounters::on_end — error path
    #[test]
    fn test_on_end_error_records_error_counter() {
        let c = counters();
        c.on_start();
        c.on_end(100, true);
        let snaps = c.provider.export();
        assert!(snaps.iter().any(|s| s.name == "edge_errors_total"));
    }

    /// @covers: RingBuffer::p99_ms
    #[test]
    fn test_ring_buffer_p99_ms_returns_correct_percentile() {
        let mut rb = RingBuffer::new(100);
        for i in 1u64..=100 { rb.push(i * 1_000); } // 1ms to 100ms in µs
        let p99 = rb.p99_ms();
        assert!(p99 >= 98.0 && p99 <= 100.0, "p99={p99}");
    }

    /// @covers: RingBuffer::p99_ms — empty
    #[test]
    fn test_ring_buffer_p99_ms_returns_zero_when_empty() {
        let rb = RingBuffer::new(64);
        assert_eq!(rb.p99_ms(), 0.0);
    }

    /// @covers: AutoscalePolicy::default
    #[test]
    fn test_autoscale_policy_defaults_are_positive() {
        let p = AutoscalePolicy::default();
        assert!(p.requests_active_max > 0);
        assert!(p.requests_per_sec_max > 0);
        assert!(p.latency_p99_ms_max > 0.0);
    }
}
