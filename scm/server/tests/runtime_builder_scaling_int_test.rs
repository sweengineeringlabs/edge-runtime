//! Integration tests for RuntimeBuilder::with_scaling().

use swe_edge_runtime::{Runtime, ScalingDecision, ScalingPolicy, ThresholdPolicy};

/// @covers: RuntimeBuilder::with_scaling
#[test]
fn test_with_scaling_accepts_threshold_policy() {
    let _builder = Runtime::builder().with_scaling(ThresholdPolicy::new(100, 500, 50.0));
}

/// @covers: RuntimeBuilder::with_scaling
#[test]
fn test_with_scaling_accepts_custom_policy() {
    #[derive(Debug)]
    struct AlwaysSteady;
    impl ScalingPolicy for AlwaysSteady {
        fn evaluate(&self, _a: u64, _r: u64, _l: f64) -> ScalingDecision {
            ScalingDecision::Steady
        }
    }
    let _builder = Runtime::builder().with_scaling(AlwaysSteady);
}

/// @covers: RuntimeBuilder::with_scaling
#[test]
fn test_with_scaling_can_be_chained_with_other_builder_methods() {
    use swe_edge_runtime::RuntimeConfig;
    let _builder = Runtime::builder()
        .config(RuntimeConfig::default())
        .with_scaling(ThresholdPolicy::new(200, 1_000, 100.0));
}
