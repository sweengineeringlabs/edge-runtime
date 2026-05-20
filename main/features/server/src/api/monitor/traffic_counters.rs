//! `TrafficCounters` — shared load state with atomic per-tick deltas.

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use swe_observ_metrics::MetricsProvider;

use crate::api::monitor::ring_buffer::RingBuffer;

const RING_CAPACITY: usize = 1024;

/// Shared load state — wraps a `MetricsProvider` for durable metric storage
/// and a ring buffer for accurate p99 latency computation.
pub struct TrafficCounters {
    pub(crate) provider: Arc<dyn MetricsProvider>,
    /// Signed so concurrent add/sub never underflows to u64::MAX.
    pub(crate) requests_in_flight: AtomicI64,
    /// Reset to 0 each tick by the background sampler.
    pub(crate) requests_since_tick: AtomicU64,
    pub(crate) errors_since_tick: AtomicU64,
    /// Ring buffer of request latencies in microseconds.
    pub(crate) latency_ring: Mutex<RingBuffer>,
}

impl TrafficCounters {
    /// Construct with the supplied metrics provider.
    pub fn new(provider: Arc<dyn MetricsProvider>) -> Self {
        Self {
            provider,
            requests_in_flight: AtomicI64::new(0),
            requests_since_tick: AtomicU64::new(0),
            errors_since_tick: AtomicU64::new(0),
            latency_ring: Mutex::new(RingBuffer::new(RING_CAPACITY)),
        }
    }

    /// Called at the start of each request.
    pub fn on_start(&self) {
        self.requests_in_flight.fetch_add(1, Ordering::Relaxed);
    }

    /// Export current metric snapshots from the underlying provider.
    pub fn export(&self) -> Vec<swe_observ_metrics::MetricSnapshot> {
        self.provider.export()
    }

    /// Called at the end of each request with measured latency and outcome.
    pub fn on_end(&self, latency_us: u64, is_error: bool) {
        self.requests_in_flight.fetch_sub(1, Ordering::Relaxed);
        self.requests_since_tick.fetch_add(1, Ordering::Relaxed);
        self.provider
            .record_counter("edge_requests_total", 1.0, &[]);
        if is_error {
            self.errors_since_tick.fetch_add(1, Ordering::Relaxed);
            self.provider.record_counter("edge_errors_total", 1.0, &[]);
        }
        self.latency_ring.lock().push(latency_us);
    }
}

/// Fluent builder for [`TrafficCounters`], allowing custom ring-buffer capacity.
struct TrafficCountersBuilder {
    provider: Arc<dyn MetricsProvider>,
    capacity: usize,
}

impl TrafficCountersBuilder {
    fn new(provider: Arc<dyn MetricsProvider>) -> Self {
        Self {
            provider,
            capacity: RING_CAPACITY,
        }
    }

    fn ring_capacity(mut self, n: usize) -> Self {
        self.capacity = n;
        self
    }

    fn build(self) -> TrafficCounters {
        TrafficCounters {
            provider: self.provider,
            requests_in_flight: AtomicI64::new(0),
            requests_since_tick: AtomicU64::new(0),
            errors_since_tick: AtomicU64::new(0),
            latency_ring: Mutex::new(RingBuffer::new(self.capacity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::monitor::shared_counters::SharedCounters;
    use std::sync::atomic::Ordering;
    use swe_observ_metrics::create_local_metrics_backend;

    fn counters() -> SharedCounters {
        Arc::new(TrafficCounters::new(Arc::new(
            create_local_metrics_backend(),
        )))
    }

    #[test]
    fn test_new_counters_start_at_zero() {
        let c = counters();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_since_tick.load(Ordering::Relaxed), 0);
    }

    /// @covers: on_start
    #[test]
    fn test_on_start_increments_in_flight() {
        let c = counters();
        c.on_start();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 1);
    }

    /// @covers: on_end
    #[test]
    fn test_on_end_decrements_in_flight_and_records_total() {
        let c = counters();
        c.on_start();
        c.on_end(500, false);
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_since_tick.load(Ordering::Relaxed), 1);
        let snaps = c.provider.export();
        assert!(snaps
            .iter()
            .any(|s| s.name == "edge_requests_total" && s.value == 1.0));
    }

    #[test]
    fn test_on_end_error_records_error_counter() {
        let c = counters();
        c.on_start();
        c.on_end(100, true);
        let snaps = c.provider.export();
        assert!(snaps.iter().any(|s| s.name == "edge_errors_total"));
    }

    /// @covers: export
    #[test]
    fn test_export_returns_recorded_snapshots() {
        let c = counters();
        c.on_start();
        c.on_end(500, false);
        let snaps = c.export();
        assert!(!snaps.is_empty());
    }

    #[test]
    fn test_traffic_counters_builder_starts_at_zero() {
        let c = TrafficCountersBuilder::new(Arc::new(create_local_metrics_backend())).build();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_since_tick.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_traffic_counters_builder_ring_capacity_is_honoured() {
        let c = TrafficCountersBuilder::new(Arc::new(create_local_metrics_backend()))
            .ring_capacity(8)
            .build();
        assert_eq!(c.latency_ring.lock().buf.len(), 8);
    }
}
