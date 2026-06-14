//! SAF — [`Validator`] re-export.
//!
//! Consumers call `.validate()` directly on any value that implements
//! [`Validator`] — there is no free-standing wrapper fn (SEA rule 191).

pub use crate::api::config::traits::validator::Validator;

/// Identifies the validator SAF contract in this crate.
pub const VALIDATOR_SVC: &str = "validator";
