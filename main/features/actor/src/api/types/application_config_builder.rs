//! Application configuration builder for the actor crate.

/// Configuration builder for actor runtime application settings.
///
/// Maps to `config/application.toml`. Use [`crate::ActorRuntime::create_config_builder`]
/// to obtain a pre-seeded builder that resolves XDG config paths.
pub struct ApplicationConfigBuilder {
    /// Optional observability tracing level override.
    pub tracing_level: Option<String>,
}

impl Default for ApplicationConfigBuilder {
    fn default() -> Self {
        Self {
            tracing_level: None,
        }
    }
}

impl ApplicationConfigBuilder {
    /// Create a new empty application config builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the observability tracing level.
    pub fn with_tracing_level(mut self, level: impl Into<String>) -> Self {
        self.tracing_level = Some(level.into());
        self
    }
}
