//! SAF — `Validator` public service surface.
pub use crate::api::runtime::traits::validator::Validator;
/// Identifies the `Validator` SAF contract in this crate.
pub const VALIDATOR_SVC: &str = "validator";
