//! `ConfigValidator` — runtime configuration validator interface.

use crate::api::runtime::errors::runtime_error::RuntimeError;
use crate::api::runtime::traits::validator::Validator;

/// Marker supertrait for runtime configuration validators.
pub trait ConfigValidator: Validator<Error = RuntimeError> {}
