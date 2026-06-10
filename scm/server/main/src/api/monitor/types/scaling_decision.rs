//! `ScalingDecision` — outcome of a scaling policy evaluation.

/// The outcome of evaluating a [`ScalingPolicy`] against current load metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingDecision {
    /// Current load exceeds at least one threshold — signal the orchestrator to scale out.
    ScaleOut,
    /// All metrics are within acceptable bounds — no scaling action required.
    Steady,
}
