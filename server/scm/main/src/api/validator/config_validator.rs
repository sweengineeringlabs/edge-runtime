//! Config validator interface — mirrors `core/validator`.

pub use crate::api::runtime::traits::config_validator::ConfigValidator;

/// Maximum field length (characters) enforced by the config validator.
pub const MAX_CONFIG_FIELD_LEN: usize = 256;
