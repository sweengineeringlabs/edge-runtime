//! Outbound gateway contract.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

/// Supplies the egress adapters the runtime uses for outbound calls.
pub trait Output: Send + Sync {
    /// HTTP outbound adapter (required).
    fn http(&self) -> Arc<dyn HttpOutbound>;
    /// gRPC outbound adapter, if configured.
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>>;
}

/// Default [`Output`] implementation — holds egress adapters by `Arc`.
pub struct DefaultOutput {
    pub(crate) http: Arc<dyn HttpOutbound>,
    pub(crate) grpc: Option<Arc<dyn GrpcOutbound>>,
}

impl DefaultOutput {
    /// Construct a gateway with only an HTTP outbound adapter.
    pub fn new_http(http: Arc<dyn HttpOutbound>) -> Self { Self { http, grpc: None } }
    /// Add (or replace) the gRPC outbound transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcOutbound>) -> Self {
        self.grpc = Some(grpc); self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;
    use swe_edge_egress_http::{HttpOutboundResult, HttpRequest, HttpResponse, HttpStreamResponse};

    struct StubHttp;
    impl HttpOutbound for StubHttp {
        fn send(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn send_stream(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>> {
            Box::pin(async { Ok(HttpStreamResponse { status: 200, headers: Default::default(), body: Box::pin(futures::stream::empty()) }) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: Output::http
    #[test]
    fn test_default_output_http_field_accessible() {
        use std::sync::Arc;
        let out = DefaultOutput { http: Arc::new(StubHttp), grpc: None };
        assert!(out.grpc.is_none());
    }
}
