//! `DefaultConfigBuilder` — fluent builder for the default subprocess isolation policy.

use crate::api::types::profile::isolator_config::IsolatorConfig;

/// Fluent builder that constructs an [`IsolatorConfig`] from the `default.toml` baseline.
///
/// Use [`DefaultConfigBuilder::new`] and call [`DefaultConfigBuilder::build`] to obtain
/// an [`IsolatorConfig`] with the built-in default profile (`"noop"`).
///
/// Operators should prefer loading config from TOML via `IsolatorSvc::create_profile_registry`
/// for production use.  This builder is provided for environments where a loader is not available.
pub(crate) struct DefaultConfigBuilder {
    config: IsolatorConfig,
}

impl DefaultConfigBuilder {
    /// Create a new builder pre-loaded with the built-in default configuration.
    #[expect(
        dead_code,
        reason = "SEA core/ anchor — wired up when factory integrates config builder"
    )]
    pub(crate) fn new() -> Self {
        Self {
            config: IsolatorConfig::default(),
        }
    }

    /// Consume the builder and return the [`IsolatorConfig`].
    #[expect(
        dead_code,
        reason = "SEA core/ anchor — wired up when factory integrates config builder"
    )]
    pub(crate) fn build(self) -> IsolatorConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_default_config_builder_new_returns_noop_profile() {
        let builder = DefaultConfigBuilder::new();
        let config = builder.build();
        assert!(config.profiles.contains_key("default"));
    }

    /// @covers: build
    #[test]
    fn test_default_config_builder_build_has_default_profile() {
        let config = DefaultConfigBuilder::new().build();
        let spec = config
            .profiles
            .get("default")
            .expect("default profile must exist");
        assert_eq!(spec.kind, "noop");
    }
}
