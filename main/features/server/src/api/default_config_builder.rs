//! Builder for runtime default configuration.
//!
//! Corresponds to `config/default.toml`.

/// Builder for runtime default (shipped) configuration.
pub struct DefaultConfigBuilder {
    _private: (),
}

impl DefaultConfigBuilder {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_builder_constructs() {
        let _b = DefaultConfigBuilder::new();
    }
}
