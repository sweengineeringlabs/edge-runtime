//! Integration tests for ScalingDecision.

use swe_edge_runtime::ScalingDecision;

/// @covers: ScalingDecision
#[test]
fn test_scaling_decision_scale_out_and_steady_are_distinct() {
    assert_ne!(ScalingDecision::ScaleOut, ScalingDecision::Steady);
}

/// @covers: ScalingDecision
#[test]
fn test_scaling_decision_is_copy() {
    let d = ScalingDecision::ScaleOut;
    let _d2 = d;
    let _d3 = d;
}

/// @covers: ScalingDecision
#[test]
fn test_scaling_decision_debug_format_is_non_empty() {
    let s = format!("{:?}", ScalingDecision::Steady);
    assert!(!s.is_empty());
}
