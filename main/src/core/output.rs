//! Default [`Output`] implementation.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

use crate::api::output::Output;

/// Holds egress adapters by `Arc`.
pub struct DefaultOutput {
    http: Arc<dyn HttpOutbound>,
    grpc: Option<Arc<dyn GrpcOutbound>>,
}

impl Output for DefaultOutput {
    fn http(&self) -> Arc<dyn HttpOutbound>         { self.http.clone() }
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>> { self.grpc.clone() }
}

impl DefaultOutput {
    /// Construct a gateway with only an HTTP outbound adapter.
    pub fn new_http(http: Arc<dyn HttpOutbound>) -> Self {
        Self { http, grpc: None }
    }

    /// Add (or replace) the gRPC outbound transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcOutbound>) -> Self {
        self.grpc = Some(grpc);
        self
    }
}
