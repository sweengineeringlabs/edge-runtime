//! `Sampler` — re-exported from the monitor traits module.

pub use crate::api::monitor::traits::sampler::Sampler;

/// Background sampling tick interval in seconds.
pub const SAMPLER_TICK_INTERVAL_SECS: u64 = 1;
