//! SEA interface contract — top-level trait index for this crate.

pub use crate::api::config_loader::ConfigLoader;
pub use crate::api::runtime_manager::RuntimeManager;
pub use crate::api::input::Input;
pub use crate::api::output::Output;

/// Validates a value against domain constraints before it is used.
///
/// Implement this trait in `core/` to express invariants that cannot
/// be captured by the type system alone (e.g. non-empty strings,
/// numeric ranges, regex patterns).
pub trait Validator {
    type Target;
    type Error;

    fn validate(&self, value: &Self::Target) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_trait_is_usable_as_concrete_impl() {
        struct AlwaysOk;
        impl Validator for AlwaysOk {
            type Target = ();
            type Error  = ();
            fn validate(&self, _: &()) -> Result<(), ()> { Ok(()) }
        }
        assert!(AlwaysOk.validate(&()).is_ok());
    }
}
