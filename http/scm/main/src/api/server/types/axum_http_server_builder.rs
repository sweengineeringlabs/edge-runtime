//! `AxumHttpServerBuilder` — fluent builder type for Axum HTTP server.

use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_http::HttpIngress;
use swe_edge_ingress_verifier::TokenVerifier;

/// Fluent builder that constructs an [`AxumHttpServer`].
pub struct AxumHttpServerBuilder {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn HttpIngress>,
    pub(crate) body_limit: usize,
    pub(crate) request_timeout: Duration,
    pub(crate) tls: Option<PemTlsConfig>,
    pub(crate) bearer_verifier: Option<Arc<dyn TokenVerifier>>,
}
