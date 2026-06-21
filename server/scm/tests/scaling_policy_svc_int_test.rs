//! Integration tests for the scaling_policy_svc SAF surface.

use swe_edge_runtime::{
    AutoscalePolicy, ScalingDecision, ScalingPolicy, ThresholdPolicy, SCALING_POLICY_SVC,
};

/// @covers: SCALING_POLICY_SVC
#[test]
fn test_scaling_policy_svc_slug_is_correct_happy() {
    assert_eq!(SCALING_POLICY_SVC, "scaling_policy");
}

// ── ScalingPolicy::evaluate ───────────────────────────────────────────────────

#[test]
fn test_evaluate_returns_steady_when_below_all_thresholds_happy() {
    let policy = ThresholdPolicy::new(100, 500, 200.0);
    assert_eq!(policy.evaluate(0, 0, 0.0), ScalingDecision::Steady);
}

#[test]
fn test_evaluate_returns_scale_out_when_active_exceeds_max_error() {
    let policy = ThresholdPolicy::new(10, 500, 200.0);
    assert_eq!(policy.evaluate(11, 0, 0.0), ScalingDecision::ScaleOut);
}

#[test]
fn test_evaluate_returns_scale_out_on_latency_breach_edge() {
    let policy = ThresholdPolicy::new(100, 500, 50.0);
    assert_eq!(policy.evaluate(0, 0, 51.0), ScalingDecision::ScaleOut);
}

// ── ScalingPolicy::build_threshold ────────────────────────────────────────────

#[test]
fn test_build_threshold_creates_threshold_policy_from_autoscale_policy_happy() {
    let ap = AutoscalePolicy {
        requests_active_max: 50,
        requests_per_sec_max: 300,
        latency_p99_ms_max: 100.0,
    };
    let policy = ThresholdPolicy::build_threshold(ap);
    assert_eq!(policy.evaluate(0, 0, 0.0), ScalingDecision::Steady);
}

#[test]
fn test_build_threshold_with_zero_limits_triggers_scale_out_error() {
    let ap = AutoscalePolicy {
        requests_active_max: 0,
        requests_per_sec_max: 0,
        latency_p99_ms_max: 0.0,
    };
    let policy = ThresholdPolicy::build_threshold(ap);
    assert_eq!(policy.evaluate(1, 0, 0.0), ScalingDecision::ScaleOut);
}

#[test]
fn test_build_threshold_with_high_limits_keeps_steady_under_load_edge() {
    let ap = AutoscalePolicy {
        requests_active_max: u64::MAX,
        requests_per_sec_max: u64::MAX,
        latency_p99_ms_max: f64::MAX,
    };
    let policy = ThresholdPolicy::build_threshold(ap);
    assert_eq!(
        policy.evaluate(1_000_000, 50_000, 5_000.0),
        ScalingDecision::Steady
    );
}
