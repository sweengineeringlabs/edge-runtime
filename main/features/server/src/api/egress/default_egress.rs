//! `DefaultEgress` — holds egress adapters by `Arc`.

use std::sync::Arc;
use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

/// Default [`Egress`] implementation — holds egress adapters by `Arc`.
pub struct DefaultEgress {
    pub(crate) http: Arc<dyn HttpEgress>,
    pub(crate) grpc: Option<Arc<dyn GrpcEgress>>,
}

impl DefaultEgress {
    /// Construct with only an HTTP outbound adapter.
    pub fn new_http(http: Arc<dyn HttpEgress>) -> Self {
        Self { http, grpc: None }
    }
    /// Add (or replace) the gRPC outbound transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcEgress>) -> Self {
        self.grpc = Some(grpc);
        self
    }
}
