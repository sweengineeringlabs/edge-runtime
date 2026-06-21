//! Integration tests for ScalingPolicy trait and ScalingDecision.

use swe_edge_runtime::{ScalingDecision, ScalingPolicy, ThresholdPolicy};

/// @covers: ScalingPolicy, ScalingDecision
#[test]
fn test_scaling_policy_is_object_safe() {
    fn _assert(_: &dyn ScalingPolicy) {}
}

/// @covers: ScalingDecision
#[test]
fn test_scaling_decision_variants_are_distinct() {
    assert_ne!(ScalingDecision::ScaleOut, ScalingDecision::Steady);
}

/// @covers: ScalingDecision
#[test]
fn test_scaling_decision_is_copy() {
    let d = ScalingDecision::ScaleOut;
    let _d2 = d;
    let _d3 = d; // copy — no move error
}

/// @covers: ThresholdPolicy, ScalingPolicy
#[test]
fn test_threshold_policy_implements_scaling_policy() {
    let policy: Box<dyn ScalingPolicy> = Box::new(ThresholdPolicy::new(100, 500, 50.0));
    assert_eq!(policy.evaluate(0, 0, 0.0), ScalingDecision::Steady);
}

/// @covers: ThresholdPolicy
#[test]
fn test_threshold_policy_new_with_zero_limits_always_scales_out() {
    let policy = ThresholdPolicy::new(0, 0, 0.0);
    // active=1 > max=0 → ScaleOut
    assert_eq!(policy.evaluate(1, 0, 0.0), ScalingDecision::ScaleOut);
}
