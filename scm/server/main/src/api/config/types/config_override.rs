//! `ConfigOverride` — partial TOML overlay applied over a `RuntimeConfig`.

use crate::api::config::errors::config_error::ConfigError;
use crate::api::config::types::runtime_config::RuntimeConfig;
use crate::api::monitor::{AutoscalePolicy, MetricsConfig};
use serde::Deserialize;
use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_http::HttpConfig;
use swe_edge_ingress_http::IngressTlsConfig;
use swe_edge_ingress_verifier::JwtConfig;
use swe_edge_observ_config::ObservabilityConfig;

/// A partial `RuntimeConfig` — all fields optional so any
/// subset of keys can be present in a TOML overlay file.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct ConfigOverride {
    pub(crate) service_name: Option<String>,
    pub(crate) http_bind: Option<String>,
    pub(crate) grpc_bind: Option<String>,
    pub(crate) shutdown_timeout_secs: Option<u64>,
    pub(crate) systemd_notify: Option<bool>,
    pub(crate) tenant_id: Option<String>,
    pub(crate) http_tls: Option<IngressTlsConfig>,
    pub(crate) grpc_tls: Option<IngressTlsConfig>,
    pub(crate) http_auth: Option<JwtConfig>,
    pub(crate) grpc_allow_unauthenticated: Option<bool>,
    pub(crate) egress_http: Option<HttpConfig>,
    pub(crate) egress_grpc: Option<GrpcChannelConfig>,
    pub(crate) grpc_reflection: Option<bool>,
    pub(crate) metrics: Option<MetricsConfig>,
    pub(crate) autoscale: Option<AutoscalePolicy>,
    pub(crate) observability: Option<ObservabilityConfig>,
}

impl ConfigOverride {
    pub(crate) fn from_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|e| ConfigError::Parse(e.to_string()))
    }

    pub(crate) fn apply_to(self, mut base: RuntimeConfig) -> RuntimeConfig {
        if let Some(v) = self.service_name {
            base.service_name = v;
        }
        if let Some(v) = self.http_bind {
            base.http_bind = v;
        }
        if let Some(v) = self.grpc_bind {
            base.grpc_bind = v;
        }
        if let Some(v) = self.shutdown_timeout_secs {
            base.shutdown_timeout_secs = v;
        }
        if let Some(v) = self.systemd_notify {
            base.systemd_notify = v;
        }
        if let Some(v) = self.tenant_id {
            if !v.is_empty() {
                base.tenant_id = Some(v);
            }
        }
        if let Some(v) = self.http_tls {
            base.http_tls = Some(v);
        }
        if let Some(v) = self.grpc_tls {
            base.grpc_tls = Some(v);
        }
        if let Some(v) = self.http_auth {
            base.http_auth = Some(v);
        }
        if let Some(v) = self.grpc_allow_unauthenticated {
            base.grpc_allow_unauthenticated = v;
        }
        if let Some(v) = self.egress_http {
            base.egress_http = Some(v);
        }
        if let Some(v) = self.egress_grpc {
            base.egress_grpc = Some(v);
        }
        if let Some(v) = self.grpc_reflection {
            base.grpc_reflection = v;
        }
        if let Some(v) = self.metrics {
            base.metrics = Some(v);
        }
        if let Some(v) = self.autoscale {
            base.autoscale = Some(v);
        }
        if let Some(v) = self.observability {
            base.observability = Some(v);
        }
        base
    }
}
