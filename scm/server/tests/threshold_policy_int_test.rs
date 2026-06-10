//! Integration tests for ThresholdPolicy.

use swe_edge_runtime::{AutoscalePolicy, ScalingDecision, ScalingPolicy, ThresholdPolicy};

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_from_autoscale_policy_preserves_thresholds() {
    let ap = AutoscalePolicy {
        requests_active_max: 300,
        requests_per_sec_max: 2_000,
        latency_p99_ms_max: 150.0,
    };
    let tp = ThresholdPolicy::from(ap);
    assert_eq!(tp.evaluate(300, 0, 0.0), ScalingDecision::Steady);
    assert_eq!(tp.evaluate(301, 0, 0.0), ScalingDecision::ScaleOut);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_steady_when_all_metrics_below_limits() {
    let policy = ThresholdPolicy::new(500, 1_000, 200.0);
    assert_eq!(policy.evaluate(499, 999, 199.9), ScalingDecision::Steady);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_scale_out_when_active_requests_exceeded() {
    let policy = ThresholdPolicy::new(500, 1_000, 200.0);
    assert_eq!(policy.evaluate(501, 0, 0.0), ScalingDecision::ScaleOut);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_scale_out_when_rps_exceeded() {
    let policy = ThresholdPolicy::new(500, 1_000, 200.0);
    assert_eq!(policy.evaluate(0, 1_001, 0.0), ScalingDecision::ScaleOut);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_scale_out_when_latency_exceeded() {
    let policy = ThresholdPolicy::new(500, 1_000, 200.0);
    assert_eq!(policy.evaluate(0, 0, 200.1), ScalingDecision::ScaleOut);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_exact_boundary_is_steady() {
    let policy = ThresholdPolicy::new(500, 1_000, 200.0);
    assert_eq!(policy.evaluate(500, 1_000, 200.0), ScalingDecision::Steady);
}
