//! `AxumHttpServerBuilder` — fluent builder type for Axum HTTP server.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_http::{HttpIngress, DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES};
use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::server::types::AxumHttpServer;

/// Fluent builder that constructs an [`AxumHttpServer`].
pub struct AxumHttpServerBuilder {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn HttpIngress>,
    pub(crate) body_limit: usize,
    pub(crate) request_timeout: Duration,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) bearer_verifier: Option<Arc<dyn TokenVerifier>>,
}

impl AxumHttpServerBuilder {
    /// Creates a builder bound to `bind` delegating requests to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            tls: None,
            bearer_verifier: None,
        }
    }

    /// Override the maximum request body size (default: [`MAX_BODY_BYTES`]).
    pub fn with_body_limit(mut self, limit: usize) -> Self {
        self.body_limit = limit;
        self
    }

    /// Override the per-request timeout (default: [`DEFAULT_REQUEST_TIMEOUT`]).
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Enable TLS or mTLS.
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Enable JWT bearer authentication.
    pub fn with_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.bearer_verifier = Some(verifier);
        self
    }

    /// Consume the builder and return a configured [`AxumHttpServer`].
    pub fn build(self) -> AxumHttpServer {
        let mut server = AxumHttpServer::new(self.bind, self.handler)
            .with_body_limit(self.body_limit)
            .with_request_timeout(self.request_timeout);
        if let Some(tls) = self.tls {
            server = server.with_tls(tls);
        }
        if let Some(verifier) = self.bearer_verifier {
            server = server.with_bearer_auth(verifier);
        }
        server
    }
}
