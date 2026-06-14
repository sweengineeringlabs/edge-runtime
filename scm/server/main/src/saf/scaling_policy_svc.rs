//! SAF — `ScalingPolicy` public service surface.
pub use crate::api::monitor::traits::scaling_policy::ScalingPolicy;
/// Identifies the `ScalingPolicy` SAF contract in this crate.
pub const SCALING_POLICY_SVC: &str = "scaling_policy";
