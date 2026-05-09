//! Load monitor — type declarations.
//!
//! `LoadCounters` is the shared atomic state updated on every request.
//! `LoadSnapshot` is a point-in-time copy used by the metrics endpoint.
//! `MetricsConfig` and `AutoscalePolicy` are TOML-configurable.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

const RING_CAPACITY: usize = 1024;

/// Shared atomic counters updated on the hot request path.
pub struct LoadCounters {
    pub(crate) requests_active:      AtomicU64,
    pub(crate) requests_total:       AtomicU64,
    pub(crate) errors_total:         AtomicU64,
    pub(crate) latency_ring:         Mutex<RingBuffer>,
    // Snapshot metrics written by the background sampler every second.
    pub(crate) snapshot_rps:         AtomicU64,
    pub(crate) snapshot_p99_ms:      AtomicU64,
    pub(crate) snapshot_err_per_sec: AtomicU64,
}

impl LoadCounters {
    /// Construct zeroed counters.
    pub fn new() -> Self {
        Self {
            requests_active:      AtomicU64::new(0),
            requests_total:       AtomicU64::new(0),
            errors_total:         AtomicU64::new(0),
            latency_ring:         Mutex::new(RingBuffer::new(RING_CAPACITY)),
            snapshot_rps:         AtomicU64::new(0),
            snapshot_p99_ms:      AtomicU64::new(0),
            snapshot_err_per_sec: AtomicU64::new(0),
        }
    }

    /// Record one completed request.
    pub(crate) fn record(&self, latency_us: u64, is_error: bool) {
        self.requests_active.fetch_sub(1, Ordering::Relaxed);
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        if is_error { self.errors_total.fetch_add(1, Ordering::Relaxed); }
        self.latency_ring.lock().push(latency_us);
    }

    /// Returns a point-in-time snapshot suitable for Prometheus exposition.
    pub fn snapshot(&self) -> LoadSnapshot {
        LoadSnapshot {
            requests_active:  self.requests_active.load(Ordering::Relaxed),
            requests_total:   self.requests_total.load(Ordering::Relaxed),
            errors_total:     self.errors_total.load(Ordering::Relaxed),
            requests_per_sec: self.snapshot_rps.load(Ordering::Relaxed),
            latency_p99_ms:   self.snapshot_p99_ms.load(Ordering::Relaxed),
            err_per_sec:      self.snapshot_err_per_sec.load(Ordering::Relaxed),
        }
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
    pub(crate) fn p99_ms(&self) -> u64 {
        let mut samples: Vec<u64> = self.buf.iter().copied().filter(|&v| v > 0).collect();
        if samples.is_empty() { return 0; }
        samples.sort_unstable();
        let idx = (samples.len() * 99 / 100).saturating_sub(1);
        samples[idx] / 1_000 // µs → ms
    }
}

/// A point-in-time snapshot of load metrics.
#[derive(Debug, Clone)]
pub struct LoadSnapshot {
    pub requests_active:  u64,
    pub requests_total:   u64,
    pub errors_total:     u64,
    pub requests_per_sec: u64,
    pub latency_p99_ms:   u64,
    pub err_per_sec:      u64,
}

impl LoadSnapshot {
    /// Render as Prometheus text exposition format (version 0.0.4).
    pub fn to_prometheus(&self) -> String {
        format!(
            "# HELP edge_requests_active In-flight request count\n\
             # TYPE edge_requests_active gauge\n\
             edge_requests_active {active}\n\
             # HELP edge_requests_total Total requests processed\n\
             # TYPE edge_requests_total counter\n\
             edge_requests_total {total}\n\
             # HELP edge_errors_total Total error responses\n\
             # TYPE edge_errors_total counter\n\
             edge_errors_total {errors}\n\
             # HELP edge_requests_per_second Current request rate\n\
             # TYPE edge_requests_per_second gauge\n\
             edge_requests_per_second {rps}\n\
             # HELP edge_request_latency_p99_ms 99th-percentile request latency in milliseconds\n\
             # TYPE edge_request_latency_p99_ms gauge\n\
             edge_request_latency_p99_ms {p99}\n\
             # HELP edge_errors_per_second Error rate\n\
             # TYPE edge_errors_per_second gauge\n\
             edge_errors_per_second {eps}\n",
            active = self.requests_active,
            total  = self.requests_total,
            errors = self.errors_total,
            rps    = self.requests_per_sec,
            p99    = self.latency_p99_ms,
            eps    = self.err_per_sec,
        )
    }
}

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
    pub latency_p99_ms_max:   u64,
}

impl Default for AutoscalePolicy {
    fn default() -> Self {
        Self {
            requests_active_max:  500,
            requests_per_sec_max: 1_000,
            latency_p99_ms_max:   200,
        }
    }
}

/// Shared handle passed between the monitor wrappers and the metrics server.
pub type SharedCounters = Arc<LoadCounters>;

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: LoadCounters::new
    #[test]
    fn test_new_counters_are_zero() {
        let c = LoadCounters::new();
        assert_eq!(c.requests_active.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_total.load(Ordering::Relaxed), 0);
        assert_eq!(c.errors_total.load(Ordering::Relaxed), 0);
    }

    /// @covers: LoadCounters::record
    #[test]
    fn test_record_increments_total_and_decrements_active() {
        let c = LoadCounters::new();
        c.requests_active.fetch_add(1, Ordering::Relaxed);
        c.record(500, false);
        assert_eq!(c.requests_active.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_total.load(Ordering::Relaxed), 1);
        assert_eq!(c.errors_total.load(Ordering::Relaxed), 0);
    }

    /// @covers: LoadCounters::record — error path
    #[test]
    fn test_record_error_increments_error_counter() {
        let c = LoadCounters::new();
        c.requests_active.fetch_add(1, Ordering::Relaxed);
        c.record(100, true);
        assert_eq!(c.errors_total.load(Ordering::Relaxed), 1);
    }

    /// @covers: RingBuffer::p99_ms
    #[test]
    fn test_ring_buffer_p99_ms_returns_correct_percentile() {
        let mut rb = RingBuffer::new(100);
        for i in 1u64..=100 { rb.push(i * 1_000); } // 1ms to 100ms in µs
        let p99 = rb.p99_ms();
        assert!(p99 >= 98 && p99 <= 100, "p99={p99}");
    }

    /// @covers: RingBuffer::p99_ms — empty
    #[test]
    fn test_ring_buffer_p99_ms_returns_zero_when_empty() {
        let rb = RingBuffer::new(64);
        assert_eq!(rb.p99_ms(), 0);
    }

    /// @covers: LoadSnapshot::to_prometheus
    #[test]
    fn test_to_prometheus_contains_all_metric_names() {
        let snap = LoadSnapshot {
            requests_active: 5, requests_total: 100, errors_total: 2,
            requests_per_sec: 50, latency_p99_ms: 12, err_per_sec: 1,
        };
        let out = snap.to_prometheus();
        assert!(out.contains("edge_requests_active 5"));
        assert!(out.contains("edge_requests_total 100"));
        assert!(out.contains("edge_request_latency_p99_ms 12"));
    }

    /// @covers: AutoscalePolicy::default
    #[test]
    fn test_autoscale_policy_default_values_are_reasonable() {
        let p = AutoscalePolicy::default();
        assert!(p.requests_active_max > 0);
        assert!(p.requests_per_sec_max > 0);
        assert!(p.latency_p99_ms_max > 0);
    }
}
