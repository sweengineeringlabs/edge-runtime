//! `ConfigOverride` — partial TOML overlay applied over a `RuntimeConfig`.

use serde::Deserialize;
use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_http::HttpConfig;
use swe_edge_ingress::IngressTlsConfig;
use swe_edge_ingress_verifier::JwtConfig;
use crate::api::config::config_error::ConfigError;
use crate::api::config::observability_config::ObservabilityConfig;
use crate::api::monitor::{AutoscalePolicy, MetricsConfig};
use crate::api::types::RuntimeConfig;

/// A partial `RuntimeConfig` — all fields optional so any
/// subset of keys can be present in a TOML overlay file.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct ConfigOverride {
    pub(crate) service_name:               Option<String>,
    pub(crate) http_bind:                  Option<String>,
    pub(crate) grpc_bind:                  Option<String>,
    pub(crate) shutdown_timeout_secs:      Option<u64>,
    pub(crate) systemd_notify:             Option<bool>,
    pub(crate) tenant_id:                  Option<String>,
    pub(crate) http_tls:                   Option<IngressTlsConfig>,
    pub(crate) grpc_tls:                   Option<IngressTlsConfig>,
    pub(crate) http_auth:                  Option<JwtConfig>,
    pub(crate) grpc_allow_unauthenticated: Option<bool>,
    pub(crate) egress_http:                Option<HttpConfig>,
    pub(crate) egress_grpc:               Option<GrpcChannelConfig>,
    pub(crate) grpc_reflection:            Option<bool>,
    pub(crate) metrics:                    Option<MetricsConfig>,
    pub(crate) autoscale:                  Option<AutoscalePolicy>,
    pub(crate) observability:              Option<ObservabilityConfig>,
}

impl ConfigOverride {
    pub(crate) fn from_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    pub(crate) fn apply_to(self, mut base: RuntimeConfig) -> RuntimeConfig {
        if let Some(v) = self.service_name              { base.service_name              = v; }
        if let Some(v) = self.http_bind                 { base.http_bind                 = v; }
        if let Some(v) = self.grpc_bind                 { base.grpc_bind                 = v; }
        if let Some(v) = self.shutdown_timeout_secs     { base.shutdown_timeout_secs     = v; }
        if let Some(v) = self.systemd_notify            { base.systemd_notify            = v; }
        if let Some(v) = self.tenant_id { if !v.is_empty() { base.tenant_id = Some(v); } }
        if let Some(v) = self.http_tls                  { base.http_tls                  = Some(v); }
        if let Some(v) = self.grpc_tls                  { base.grpc_tls                  = Some(v); }
        if let Some(v) = self.http_auth                 { base.http_auth                 = Some(v); }
        if let Some(v) = self.grpc_allow_unauthenticated { base.grpc_allow_unauthenticated = v; }
        if let Some(v) = self.egress_http               { base.egress_http               = Some(v); }
        if let Some(v) = self.egress_grpc               { base.egress_grpc               = Some(v); }
        if let Some(v) = self.grpc_reflection            { base.grpc_reflection           = v; }
        if let Some(v) = self.metrics                    { base.metrics                   = Some(v); }
        if let Some(v) = self.autoscale                  { base.autoscale                 = Some(v); }
        if let Some(v) = self.observability              { base.observability             = Some(v); }
        base
    }
}

/// Fluent builder for [`ConfigOverride`].
struct ConfigOverrideBuilder {
    inner: ConfigOverride,
}

impl ConfigOverrideBuilder {
    fn new() -> Self { Self { inner: ConfigOverride::default() } }
    fn service_name(mut self, v: impl Into<String>) -> Self { self.inner.service_name = Some(v.into()); self }
    fn http_bind(mut self, v: impl Into<String>) -> Self { self.inner.http_bind = Some(v.into()); self }
    fn grpc_bind(mut self, v: impl Into<String>) -> Self { self.inner.grpc_bind = Some(v.into()); self }
    fn shutdown_timeout_secs(mut self, v: u64) -> Self { self.inner.shutdown_timeout_secs = Some(v); self }
    fn systemd_notify(mut self, v: bool) -> Self { self.inner.systemd_notify = Some(v); self }
    fn tenant_id(mut self, v: impl Into<String>) -> Self { self.inner.tenant_id = Some(v.into()); self }
    fn grpc_allow_unauthenticated(mut self, v: bool) -> Self { self.inner.grpc_allow_unauthenticated = Some(v); self }
    fn grpc_reflection(mut self, v: bool) -> Self { self.inner.grpc_reflection = Some(v); self }
    fn build(self) -> ConfigOverride { self.inner }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_parses_partial_toml() {
        let o = ConfigOverride::from_str(r#"http_bind = "127.0.0.1:9090""#).unwrap();
        assert_eq!(o.http_bind.as_deref(), Some("127.0.0.1:9090"));
    }

    #[test]
    fn test_from_str_invalid_toml_returns_error() {
        assert!(ConfigOverride::from_str("not = [valid toml").is_err());
    }

    #[test]
    fn test_apply_to_merges_set_fields() {
        let base = RuntimeConfig::default();
        let o = ConfigOverride::from_str(r#"service_name = "acme""#).unwrap();
        let merged = o.apply_to(base);
        assert_eq!(merged.service_name, "acme");
    }

    #[test]
    fn test_apply_to_skips_empty_tenant_id() {
        let base = RuntimeConfig::default();
        let o = ConfigOverride::from_str(r#"tenant_id = """#).unwrap();
        let merged = o.apply_to(base);
        assert!(merged.tenant_id.is_none());
    }

    #[test]
    fn test_apply_to_propagates_observability_section() {
        let base = RuntimeConfig::default();
        let toml = r#"
            [observability.tracing]
            level  = "debug"
            format = "json"
        "#;
        let o = ConfigOverride::from_str(toml).unwrap();
        let merged = o.apply_to(base);
        let tracing = &merged.observability.unwrap().tracing;
        assert_eq!(tracing.level, crate::api::config::tracing_level::TracingLevel::Debug);
    }

    #[test]
    fn test_config_override_builder_sets_fields() {
        let o = ConfigOverrideBuilder::new()
            .service_name("my-svc")
            .http_bind("0.0.0.0:9000")
            .shutdown_timeout_secs(60)
            .build();
        assert_eq!(o.service_name.as_deref(), Some("my-svc"));
        assert_eq!(o.http_bind.as_deref(), Some("0.0.0.0:9000"));
        assert_eq!(o.shutdown_timeout_secs, Some(60));
        assert!(o.grpc_bind.is_none());
    }
}
