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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;
    use swe_edge_egress_http::{HttpEgressResult, HttpRequest, HttpResponse, HttpStreamResponse};

    struct StubHttp;
    impl HttpEgress for StubHttp {
        fn send(&self, _: HttpRequest) -> BoxFuture<'_, HttpEgressResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn send_stream(
            &self,
            _: HttpRequest,
        ) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>> {
            Box::pin(async {
                Ok(HttpStreamResponse {
                    status: 200,
                    headers: Default::default(),
                    body: Box::pin(futures::stream::empty()),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>> {
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
            GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcRequest, GrpcResponse,
            GrpcStatusCode,
        };
        struct StubGrpc;
        impl GrpcEgress for StubGrpc {
            fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
                Box::pin(async {
                    Err(GrpcEgressError::Status(
                        GrpcStatusCode::Unavailable,
                        "stub".into(),
                    ))
                })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
                Box::pin(async { Ok(()) })
            }
        }
        let out = DefaultEgress::new_http(Arc::new(StubHttp)).with_grpc(Arc::new(StubGrpc));
        assert!(out.grpc.is_some());
    }
}
