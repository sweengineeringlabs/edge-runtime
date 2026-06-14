//! SAF — `Sampler` public service surface.
pub use crate::api::monitor::traits::sampler::Sampler;
/// Identifies the `Sampler` SAF contract in this crate.
pub const SAMPLER_SVC: &str = "sampler";
