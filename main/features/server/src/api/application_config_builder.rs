//! Builder for runtime application configuration.
//!
//! Corresponds to `config/application.toml`.

/// Builder for runtime application configuration.
///
/// Construct via [`crate::saf::builder`]. Finalize with [`ApplicationConfigBuilder::build`].
pub struct ApplicationConfigBuilder {
    _private: (),
}

impl ApplicationConfigBuilder {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_constructs() {
        let _b = ApplicationConfigBuilder::new();
    }
}
