//! Integration tests for the sampler_svc SAF surface.

use std::sync::Arc;
use swe_edge_runtime::{Sampler, SharedCounters, SAMPLER_SVC};
use swe_observ_metrics::create_local_metrics_backend;

struct NoopSampler;
impl Sampler for NoopSampler {}

/// @covers: SAMPLER_SVC
#[test]
fn test_sampler_svc_slug_is_correct_happy() {
    assert_eq!(SAMPLER_SVC, "sampler");
}

// ── Sampler::make_counters ────────────────────────────────────────────────────

#[test]
fn test_make_counters_creates_traffic_counters_from_provider_happy() {
    let provider = Arc::new(create_local_metrics_backend());
    let _counters = NoopSampler::make_counters(provider);
}

#[test]
fn test_make_counters_with_noop_provider_exports_empty_snapshot_error() {
    let provider = Arc::new(create_local_metrics_backend());
    let counters = NoopSampler::make_counters(provider);
    assert!(counters.export().is_empty());
}

#[test]
fn test_make_counters_on_start_does_not_panic_edge() {
    let provider = Arc::new(create_local_metrics_backend());
    let counters = NoopSampler::make_counters(provider);
    counters.on_start();
}

// ── Sampler::share_counters ───────────────────────────────────────────────────

#[test]
fn test_share_counters_wraps_traffic_counters_in_arc_happy() {
    let provider = Arc::new(create_local_metrics_backend());
    let tc = NoopSampler::make_counters(provider);
    let shared: SharedCounters = NoopSampler::share_counters(tc);
    assert_eq!(Arc::strong_count(&shared), 1);
}

#[test]
fn test_share_counters_strong_count_increases_when_cloned_error() {
    let provider = Arc::new(create_local_metrics_backend());
    let tc = NoopSampler::make_counters(provider);
    let shared = NoopSampler::share_counters(tc);
    let clone = Arc::clone(&shared);
    assert_eq!(Arc::strong_count(&shared), 2);
    drop(clone);
}

#[test]
fn test_share_counters_shared_arc_exports_empty_on_fresh_counters_edge() {
    let provider = Arc::new(create_local_metrics_backend());
    let tc = NoopSampler::make_counters(provider);
    let shared = NoopSampler::share_counters(tc);
    assert!(shared.export().is_empty());
}

// ── Sampler::make_ring_buffer ─────────────────────────────────────────────────

#[test]
fn test_make_ring_buffer_p99_is_zero_on_empty_buffer_happy() {
    let buf = NoopSampler::make_ring_buffer(16);
    assert_eq!(buf.p99_ms(), 0.0);
}

#[test]
fn test_make_ring_buffer_p99_is_nonzero_after_push_error() {
    let mut buf = NoopSampler::make_ring_buffer(16);
    buf.push(5_000); // 5000µs = 5ms
    assert!(buf.p99_ms() > 0.0);
}

#[test]
fn test_make_ring_buffer_single_element_p99_matches_pushed_value_edge() {
    let mut buf = NoopSampler::make_ring_buffer(16);
    buf.push(2_000); // 2000µs = 2ms
    assert_eq!(buf.p99_ms(), 2.0);
}
