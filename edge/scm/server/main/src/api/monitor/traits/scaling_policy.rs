//! `ScalingPolicy` — port contract for autoscale threshold evaluation.

use crate::api::monitor::types::autoscale_policy::AutoscalePolicy;
use crate::api::monitor::types::scaling_decision::ScalingDecision;
use crate::api::monitor::types::threshold_policy::ThresholdPolicy;

/// Evaluates current load metrics and signals when horizontal scaling is needed.
///
/// Implementations are polled once per second by the background sampler.
/// Return [`ScalingDecision::ScaleOut`](crate::ScalingDecision::ScaleOut) to emit a scale-out signal;
/// [`ScalingDecision::Steady`](crate::ScalingDecision::Steady) means no action.
pub trait ScalingPolicy: Send + Sync + std::fmt::Debug {
    /// Evaluate the current load snapshot.
    ///
    /// # Parameters
    /// - `active` — in-flight requests at sampling time
    /// - `rps` — requests completed since the last 1-second tick
    /// - `latency_p99_ms` — p99 request latency in milliseconds
    fn evaluate(&self, active: u64, rps: u64, latency_p99_ms: f64) -> ScalingDecision;

    /// Build a [`ThresholdPolicy`](crate::ThresholdPolicy) from an [`AutoscalePolicy`](crate::AutoscalePolicy) configuration.
    fn build_threshold(policy: AutoscalePolicy) -> ThresholdPolicy
    where
        Self: Sized,
    {
        ThresholdPolicy::from(policy)
    }
}
