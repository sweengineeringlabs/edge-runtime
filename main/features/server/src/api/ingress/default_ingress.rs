//! `DefaultIngress` — holds ingress adapters by `Arc`.

use std::sync::Arc;
use swe_edge_ingress::{GrpcInbound, HttpInbound};

/// Default [`Ingress`] implementation — holds optional ingress adapters by `Arc`.
pub struct DefaultIngress {
    pub(crate) http: Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc: Option<Arc<dyn GrpcInbound>>,
}

impl DefaultIngress {
    /// Start with HTTP as the sole transport.
    pub fn new_http(http: Arc<dyn HttpInbound>) -> Self {
        Self {
            http: Some(http),
            grpc: None,
        }
    }
    /// Start with gRPC as the sole transport.
    pub fn new_grpc(grpc: Arc<dyn GrpcInbound>) -> Self {
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
    pub fn with_http(mut self, http: Arc<dyn HttpInbound>) -> Self {
        self.http = Some(http);
        self
    }
    /// Add (or replace) the gRPC transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcInbound>) -> Self {
        self.grpc = Some(grpc);
        self
    }
}

impl crate::api::ingress::Ingress for DefaultIngress {
    fn http(&self) -> Option<Arc<dyn HttpInbound>> {
        self.http.clone()
    }
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> {
        self.grpc.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: empty
    #[test]
    fn test_empty_has_no_transports() {
        let i = DefaultIngress::empty();
        assert!(i.http.is_none() && i.grpc.is_none());
    }

    /// @covers: new_http
    #[test]
    fn test_new_http_sets_http_transport() {
        use edge_domain::RequestContext;
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpHealthCheck, HttpInboundResult, HttpRequest, HttpResponse};
        struct Stub;
        impl HttpInbound for Stub {
            fn handle(
                &self,
                _: HttpRequest,
                _: RequestContext,
            ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        let i = DefaultIngress::new_http(Arc::new(Stub));
        assert!(i.http.is_some() && i.grpc.is_none());
    }

    /// @covers: with_http
    #[test]
    fn test_with_http_adds_transport() {
        use edge_domain::RequestContext;
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpHealthCheck, HttpInboundResult, HttpRequest, HttpResponse};
        struct Stub;
        impl HttpInbound for Stub {
            fn handle(
                &self,
                _: HttpRequest,
                _: RequestContext,
            ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        let i = DefaultIngress::empty().with_http(Arc::new(Stub));
        assert!(i.http.is_some());
    }

    /// @covers: new_grpc
    #[test]
    fn test_new_grpc_sets_grpc_transport() {
        use edge_domain::RequestContext;
        use futures::future::BoxFuture;
        use swe_edge_ingress::{
            GrpcHealthCheck, GrpcInboundError, GrpcInboundResult, GrpcMessageStream, GrpcMetadata,
            GrpcRequest, GrpcResponse,
        };
        struct Stub;
        impl GrpcInbound for Stub {
            fn handle_unary(
                &self,
                _: GrpcRequest,
                _: RequestContext,
            ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn handle_stream(
                &self,
                _: String,
                _: GrpcMetadata,
                _: GrpcMessageStream,
                _: RequestContext,
            ) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
                Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
            }
        }
        let i = DefaultIngress::new_grpc(Arc::new(Stub));
        assert!(i.grpc.is_some() && i.http.is_none());
    }

    /// @covers: with_grpc
    #[test]
    fn test_with_grpc_adds_transport() {
        use edge_domain::RequestContext;
        use futures::future::BoxFuture;
        use swe_edge_ingress::{
            GrpcHealthCheck, GrpcInboundError, GrpcInboundResult, GrpcMessageStream, GrpcMetadata,
            GrpcRequest, GrpcResponse,
        };
        struct Stub;
        impl GrpcInbound for Stub {
            fn handle_unary(
                &self,
                _: GrpcRequest,
                _: RequestContext,
            ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn handle_stream(
                &self,
                _: String,
                _: GrpcMetadata,
                _: GrpcMessageStream,
                _: RequestContext,
            ) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
                Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
            }
        }
        let i = DefaultIngress::empty().with_grpc(Arc::new(Stub));
        assert!(i.grpc.is_some());
    }
}
