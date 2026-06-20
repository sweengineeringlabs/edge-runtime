//! `Sampler` — background metric sampling loop interface.

use std::sync::Arc;

use swe_observ_metrics::MetricsProvider;

use crate::api::monitor::types::ring_buffer::RingBuffer;
use crate::api::monitor::types::shared_counters::SharedCounters;
use crate::api::monitor::types::traffic_counters::TrafficCounters;

/// Marker trait for types that run a background metric-sampling loop.
pub trait Sampler: Send + Sync {
    /// Return the sampler's source identifier for observability labels.
    fn sampler_name(&self) -> &'static str {
        "sampler"
    }

    /// Create a new [`TrafficCounters`] backed by the given metrics provider.
    fn make_counters(provider: Arc<dyn MetricsProvider>) -> TrafficCounters
    where
        Self: Sized,
    {
        TrafficCounters::new(provider)
    }

    /// Wrap a [`TrafficCounters`] in an [`Arc`] to produce a [`SharedCounters`].
    fn share_counters(tc: TrafficCounters) -> SharedCounters
    where
        Self: Sized,
    {
        Arc::new(tc)
    }

    /// Allocate a [`RingBuffer`] of the given capacity for latency sampling.
    fn make_ring_buffer(capacity: usize) -> RingBuffer
    where
        Self: Sized,
    {
        RingBuffer::new(capacity)
    }
}
