//! Application configuration builder for the actor crate.

/// Configuration builder for actor runtime application settings.
///
/// Maps to `config/application.toml`. Use [`crate::ActorRuntime::create_config_builder`]
/// to obtain a pre-seeded builder that resolves XDG config paths.
#[derive(Default)]
pub struct ApplicationConfigBuilder {
    /// Optional observability tracing level override.
    pub tracing_level: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Create a new empty application config builder.
    #[expect(
        dead_code,
        reason = "SEA api/ anchor — exported for consumers, not used internally"
    )]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the observability tracing level.
    #[expect(
        dead_code,
        reason = "SEA api/ anchor — exported for consumers, not used internally"
    )]
    pub fn with_tracing_level(mut self, level: impl Into<String>) -> Self {
        self.tracing_level = Some(level.into());
        self
    }
}
