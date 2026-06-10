//! `ThresholdPolicy` — threshold-based [`ScalingPolicy`] implementation.

use crate::api::monitor::traits::scaling_policy::ScalingPolicy;
use crate::api::monitor::types::autoscale_policy::AutoscalePolicy;
use crate::api::monitor::types::scaling_decision::ScalingDecision;

/// A threshold-based scaling policy: signals scale-out when any metric exceeds
/// its configured ceiling.
///
/// Construct via [`ThresholdPolicy::new`] for programmatic use, or via
/// [`From<AutoscalePolicy>`] to derive limits from a TOML-loaded config.
#[derive(Debug, Clone)]
pub struct ThresholdPolicy {
    requests_active_max: u64,
    requests_per_sec_max: u64,
    latency_p99_ms_max: f64,
}

impl ThresholdPolicy {
    /// Create a new threshold policy with explicit limits.
    ///
    /// # Parameters
    /// - `requests_active_max` — max concurrent in-flight requests
    /// - `requests_per_sec_max` — max requests per second
    /// - `latency_p99_ms_max` — max p99 latency in milliseconds
    pub fn new(
        requests_active_max: u64,
        requests_per_sec_max: u64,
        latency_p99_ms_max: f64,
    ) -> Self {
        Self {
            requests_active_max,
            requests_per_sec_max,
            latency_p99_ms_max,
        }
    }
}

impl From<AutoscalePolicy> for ThresholdPolicy {
    fn from(p: AutoscalePolicy) -> Self {
        Self::new(
            p.requests_active_max,
            p.requests_per_sec_max,
            p.latency_p99_ms_max,
        )
    }
}

impl ScalingPolicy for ThresholdPolicy {
    fn evaluate(&self, active: u64, rps: u64, latency_p99_ms: f64) -> ScalingDecision {
        if active > self.requests_active_max
            || rps > self.requests_per_sec_max
            || latency_p99_ms > self.latency_p99_ms_max
        {
            ScalingDecision::ScaleOut
        } else {
            ScalingDecision::Steady
        }
    }
}
