//! Builder for runtime default configuration.
//!
//! Corresponds to `config/default.toml`.

/// Builder for runtime default (shipped) configuration.
pub struct DefaultConfigBuilder {
    service_name: Option<String>,
    http_bind: Option<String>,
    grpc_bind: Option<String>,
    shutdown_timeout_secs: Option<u64>,
    systemd_notify: Option<bool>,
    tenant_id: Option<String>,
    grpc_allow_unauthenticated: Option<bool>,
    grpc_reflection: Option<bool>,
}

impl DefaultConfigBuilder {
    /// Create a new default config builder.
    pub fn new() -> Self {
        Self {
            service_name: None,
            http_bind: None,
            grpc_bind: None,
            shutdown_timeout_secs: None,
            systemd_notify: None,
            tenant_id: None,
            grpc_allow_unauthenticated: None,
            grpc_reflection: None,
        }
    }

    /// Set the service name.
    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.service_name = Some(name.into());
        self
    }

    /// Set the HTTP bind address.
    pub fn with_http_bind(mut self, addr: impl Into<String>) -> Self {
        self.http_bind = Some(addr.into());
        self
    }

    /// Set the gRPC bind address.
    pub fn with_grpc_bind(mut self, addr: impl Into<String>) -> Self {
        self.grpc_bind = Some(addr.into());
        self
    }

    /// Set the shutdown timeout in seconds.
    pub fn with_shutdown_timeout_secs(mut self, secs: u64) -> Self {
        self.shutdown_timeout_secs = Some(secs);
        self
    }

    /// Enable or disable systemd notify.
    pub fn with_systemd_notify(mut self, enabled: bool) -> Self {
        self.systemd_notify = Some(enabled);
        self
    }

    /// Set the tenant ID.
    pub fn with_tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    /// Allow unauthenticated gRPC requests.
    pub fn with_grpc_allow_unauthenticated(mut self, allow: bool) -> Self {
        self.grpc_allow_unauthenticated = Some(allow);
        self
    }

    /// Enable gRPC reflection.
    pub fn with_grpc_reflection(mut self, enabled: bool) -> Self {
        self.grpc_reflection = Some(enabled);
        self
    }

    /// Build the configuration.
    pub fn build(self) -> Self {
        self
    }
}

impl Default for DefaultConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_builder_constructs() {
        let _b = DefaultConfigBuilder::new();
    }

    #[test]
    fn test_with_service_name() {
        let b = DefaultConfigBuilder::new().with_service_name("test-service");
        assert_eq!(b.service_name.as_deref(), Some("test-service"));
    }

    #[test]
    fn test_with_http_bind() {
        let b = DefaultConfigBuilder::new().with_http_bind("127.0.0.1:8080");
        assert_eq!(b.http_bind.as_deref(), Some("127.0.0.1:8080"));
    }

    #[test]
    fn test_fluent_chain() {
        let b = DefaultConfigBuilder::new()
            .with_service_name("my-service")
            .with_http_bind("0.0.0.0:9000")
            .with_shutdown_timeout_secs(60);
        assert_eq!(b.service_name.as_deref(), Some("my-service"));
        assert_eq!(b.http_bind.as_deref(), Some("0.0.0.0:9000"));
        assert_eq!(b.shutdown_timeout_secs, Some(60));
    }
}
