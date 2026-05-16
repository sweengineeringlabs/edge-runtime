//! `DefaultEgress` — holds egress adapters by `Arc`.

use std::sync::Arc;
use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

/// Default [`Egress`] implementation — holds egress adapters by `Arc`.
pub struct DefaultEgress {
    pub(crate) http: Arc<dyn HttpOutbound>,
    pub(crate) grpc: Option<Arc<dyn GrpcOutbound>>,
}

impl DefaultEgress {
    /// Construct with only an HTTP outbound adapter.
    pub fn new_http(http: Arc<dyn HttpOutbound>) -> Self {
        Self { http, grpc: None }
    }
    /// Add (or replace) the gRPC outbound transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcOutbound>) -> Self {
        self.grpc = Some(grpc);
        self
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
        fn send_stream(
            &self,
            _: HttpRequest,
        ) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>> {
            Box::pin(async {
                Ok(HttpStreamResponse {
                    status: 200,
                    headers: Default::default(),
                    body: Box::pin(futures::stream::empty()),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: new_http
    #[test]
    fn test_new_http_creates_egress_with_no_grpc() {
        let out = DefaultEgress::new_http(Arc::new(StubHttp));
        assert!(out.grpc.is_none());
    }

    /// @covers: with_grpc
    #[test]
    fn test_with_grpc_sets_grpc_adapter() {
        use swe_edge_egress_grpc::{
            GrpcOutbound, GrpcOutboundError, GrpcOutboundResult, GrpcRequest, GrpcResponse,
            GrpcStatusCode,
        };
        struct StubGrpc;
        impl GrpcOutbound for StubGrpc {
            fn call_unary(
                &self,
                _: GrpcRequest,
            ) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
                Box::pin(async {
                    Err(GrpcOutboundError::Status(
                        GrpcStatusCode::Unavailable,
                        "stub".into(),
                    ))
                })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
                Box::pin(async { Ok(()) })
            }
        }
        let out = DefaultEgress::new_http(Arc::new(StubHttp)).with_grpc(Arc::new(StubGrpc));
        assert!(out.grpc.is_some());
    }
}
