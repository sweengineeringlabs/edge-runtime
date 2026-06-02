//! `DefaultConfigBuilder` — builder for the default resource policy configuration.

/// Builds the default resource policy configuration from `config/default.toml`.
///
/// Used by the runtime to construct the baseline [`ResourcePolicyConfig`]
/// before consumer overrides are applied.
///
/// [`ResourcePolicyConfig`]: crate::ResourcePolicyConfig
#[derive(Debug, Default)]
pub(crate) struct DefaultConfigBuilder;

impl DefaultConfigBuilder {
    /// Create a new instance.
    #[expect(
        dead_code,
        reason = "SEA core/ anchor — wired up when factory integrates config builder"
    )]
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_builder_new_creates_instance() {
        let _ = DefaultConfigBuilder::new();
    }
}
