//! `DefaultThresholdPolicy` — threshold-based [`ScalingPolicy`] core implementation.

use crate::api::monitor::traits::scaling_policy::ScalingPolicy;
use crate::api::monitor::types::scaling_decision::ScalingDecision;

/// Core threshold-based scaling policy: signals scale-out when any metric
/// exceeds its configured ceiling.
pub(crate) struct DefaultThresholdPolicy {
    requests_active_max: u64,
    requests_per_sec_max: u64,
    latency_p99_ms_max: f64,
}

impl DefaultThresholdPolicy {
    pub(crate) fn new(
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

impl std::fmt::Debug for DefaultThresholdPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultThresholdPolicy")
            .field("requests_active_max", &self.requests_active_max)
            .field("requests_per_sec_max", &self.requests_per_sec_max)
            .field("latency_p99_ms_max", &self.latency_p99_ms_max)
            .finish()
    }
}

impl ScalingPolicy for DefaultThresholdPolicy {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_thresholds_reflected_by_evaluate_happy() {
        let policy = DefaultThresholdPolicy::new(10, 20, 5.0);
        assert_eq!(policy.evaluate(5, 10, 2.0), ScalingDecision::Steady);
    }

    #[test]
    fn test_evaluate_steady_when_all_metrics_below_threshold_happy() {
        let policy = DefaultThresholdPolicy::new(100, 200, 50.0);
        assert_eq!(policy.evaluate(50, 100, 25.0), ScalingDecision::Steady);
    }

    #[test]
    fn test_evaluate_scale_out_when_active_requests_exceeded_error() {
        let policy = DefaultThresholdPolicy::new(100, 200, 50.0);
        assert_eq!(policy.evaluate(101, 0, 0.0), ScalingDecision::ScaleOut);
    }

    #[test]
    fn test_evaluate_steady_when_all_metrics_at_exact_threshold_edge() {
        let policy = DefaultThresholdPolicy::new(100, 200, 50.0);
        assert_eq!(policy.evaluate(100, 200, 50.0), ScalingDecision::Steady);
    }
}
