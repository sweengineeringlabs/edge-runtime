//! `AxumHttpServer` — public API type for the Axum-backed HTTP server.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_http::{HttpIngress, HttpStream};
use swe_edge_ingress_tls::IngressTlsConfig;
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
