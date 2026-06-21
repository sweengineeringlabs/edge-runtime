//! No-op [`Validator`] implementation.

use swe_edge_ingress_http::Validator;

use crate::api::NoopValidator;

impl Validator for NoopValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
