//! `ConfigValidator` — runtime configuration validator interface.

use crate::api::error::RuntimeError;
use crate::api::traits::Validator;
use crate::api::types::RuntimeConfig;

/// Marker supertrait for `RuntimeConfig` validators.
pub trait ConfigValidator: Validator<Target = RuntimeConfig, Error = RuntimeError> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validator_is_object_safe() {
        fn _assert(_: &dyn ConfigValidator) {}
    }
}
