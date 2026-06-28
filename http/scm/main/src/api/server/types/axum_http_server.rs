//! `AxumHttpServer` — public API type for the Axum-backed HTTP server.

use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_http::{HttpIngress, HttpStream, DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES};
use swe_edge_ingress_verifier::TokenVerifier;

/// Axum-based HTTP server that routes all inbound requests through an
/// [`HttpIngress`] port.
pub struct AxumHttpServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn HttpIngress>,
    pub(crate) body_limit: usize,
    pub(crate) request_timeout: Duration,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) bearer_verifier: Option<Arc<dyn TokenVerifier>>,
    pub(crate) stream_handler: Option<Arc<dyn HttpStream>>,
}

impl AxumHttpServer {
    /// Create a server binding to `bind` and delegating requests to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            tls: None,
            bearer_verifier: None,
            stream_handler: None,
        }
    }

    /// Attach an [`HttpStream`] handler for SSE and WebSocket requests.
    pub fn with_stream_handler(mut self, handler: Arc<dyn HttpStream>) -> Self {
        self.stream_handler = Some(handler);
        self
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
}
