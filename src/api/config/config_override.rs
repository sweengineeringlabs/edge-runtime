//! `ConfigOverride` — partial TOML overlay applied over a `RuntimeConfig`.

use serde::Deserialize;
use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_http::HttpConfig;
use swe_edge_ingress::IngressTlsConfig;
use swe_edge_ingress_verifier::JwtConfig;
use crate::api::monitor::{AutoscalePolicy, MetricsConfig};
use crate::api::types::RuntimeConfig;
use crate::api::config::config_error::ConfigError;

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
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_str
    #[test]
    fn test_from_str_parses_partial_toml() {
        let o = ConfigOverride::from_str(r#"http_bind = "127.0.0.1:9090""#).unwrap();
        assert_eq!(o.http_bind.as_deref(), Some("127.0.0.1:9090"));
    }

    /// @covers: from_str
    #[test]
    fn test_from_str_invalid_toml_returns_error() {
        assert!(ConfigOverride::from_str("not = [valid toml").is_err());
    }

    /// @covers: apply_to
    #[test]
    fn test_apply_to_merges_set_fields() {
        let base = RuntimeConfig::default();
        let o = ConfigOverride::from_str(r#"service_name = "acme""#).unwrap();
        let merged = o.apply_to(base);
        assert_eq!(merged.service_name, "acme");
    }

    /// @covers: apply_to
    #[test]
    fn test_apply_to_skips_empty_tenant_id() {
        let base = RuntimeConfig::default();
        let o = ConfigOverride::from_str(r#"tenant_id = """#).unwrap();
        let merged = o.apply_to(base);
        assert!(merged.tenant_id.is_none());
    }
}
