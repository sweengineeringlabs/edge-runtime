//! SAF — `ConfigValidator` public service surface.
pub use crate::api::runtime::traits::config_validator::ConfigValidator;
/// Identifies the `ConfigValidator` SAF contract in this crate.
pub const CONFIG_VALIDATOR_SVC: &str = "config_validator";
