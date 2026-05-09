use std::sync::atomic::Ordering;

use crate::api::monitor::{AutoscalePolicy, SharedCounters};

/// Ticks every second: pushes derived gauges into the provider and checks
/// autoscale thresholds.
pub(crate) struct BackgroundSampler {
    counters: SharedCounters,
    policy:   Option<AutoscalePolicy>,
}

impl BackgroundSampler {
    pub(crate) fn new(counters: SharedCounters, policy: Option<AutoscalePolicy>) -> Self {
        Self { counters, policy }
    }

    pub(crate) async fn run(self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;

            let active = self.counters.requests_in_flight.load(Ordering::Relaxed) as f64;
            let rps    = self.counters.requests_since_tick.swap(0, Ordering::Relaxed) as f64;
            let eps    = self.counters.errors_since_tick.swap(0, Ordering::Relaxed) as f64;
            let p99    = self.counters.latency_ring.lock().p99_ms();

            let p = &*self.counters.provider;
            p.record_gauge("edge_requests_active",        active, &[]);
            p.record_gauge("edge_requests_per_second",    rps,    &[]);
            p.record_gauge("edge_errors_per_second",      eps,    &[]);
            p.record_gauge("edge_request_latency_p99_ms", p99,    &[]);

            if let Some(ref policy) = self.policy {
                if active as u64 > policy.requests_active_max {
                    tracing::warn!(active, max = policy.requests_active_max,
                        "scale-out signal: requests_active exceeded threshold");
                }
                if rps as u64 > policy.requests_per_sec_max {
                    tracing::warn!(rps, max = policy.requests_per_sec_max,
                        "scale-out signal: requests_per_second exceeded threshold");
                }
                if p99 > policy.latency_p99_ms_max {
                    tracing::warn!(p99_ms = p99, max = policy.latency_p99_ms_max,
                        "scale-out signal: latency_p99_ms exceeded threshold");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use swe_observ_metrics::create_local_metrics_backend;
    use crate::api::monitor::LoadCounters;

    fn counters() -> SharedCounters {
        Arc::new(LoadCounters::new(Arc::new(create_local_metrics_backend())))
    }

    /// @covers: BackgroundSampler::new
    #[test]
    fn test_background_sampler_new_does_not_panic() {
        let _s = BackgroundSampler::new(counters(), None);
    }
}
