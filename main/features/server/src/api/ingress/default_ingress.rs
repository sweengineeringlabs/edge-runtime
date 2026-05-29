//! `DefaultIngress` — holds ingress adapters by `Arc`.

use std::sync::Arc;
use swe_edge_ingress_grpc::GrpcIngress;
use swe_edge_ingress_http::HttpIngress;

/// Default [`Ingress`] implementation — holds optional ingress adapters by `Arc`.
pub struct DefaultIngress {
    pub(crate) http: Option<Arc<dyn HttpIngress>>,
    pub(crate) grpc: Option<Arc<dyn GrpcIngress>>,
}

impl DefaultIngress {
    /// Start with HTTP as the sole transport.
    pub fn new_http(http: Arc<dyn HttpIngress>) -> Self {
        Self {
            http: Some(http),
            grpc: None,
        }
    }
    /// Start with gRPC as the sole transport.
    pub fn new_grpc(grpc: Arc<dyn GrpcIngress>) -> Self {
        Self {
            http: None,
            grpc: Some(grpc),
        }
    }
    /// Construct with no transports configured.
    pub fn empty() -> Self {
        Self {
            http: None,
            grpc: None,
        }
    }
    /// Add (or replace) the HTTP transport.
    pub fn with_http(mut self, http: Arc<dyn HttpIngress>) -> Self {
        self.http = Some(http);
        self
    }
    /// Add (or replace) the gRPC transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcIngress>) -> Self {
        self.grpc = Some(grpc);
        self
    }
}

impl crate::api::ingress::Ingress for DefaultIngress {
    fn http(&self) -> Option<Arc<dyn HttpIngress>> {
        self.http.clone()
    }
    fn grpc(&self) -> Option<Arc<dyn GrpcIngress>> {
        self.grpc.clone()
    }
}
