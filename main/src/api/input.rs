//! Inbound gateway contract.

use std::sync::Arc;

use swe_edge_ingress::{GrpcInbound, HttpInbound};

/// Supplies the ingress adapters the runtime binds traffic through.
pub trait Input: Send + Sync {
    /// HTTP inbound adapter, if configured.
    fn http(&self) -> Option<Arc<dyn HttpInbound>>;
    /// gRPC inbound adapter, if configured.
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>>;
    /// Returns `true` if at least one transport is configured.
    fn has_any(&self) -> bool {
        self.http().is_some() || self.grpc().is_some()
    }
}

/// Default [`Input`] implementation — holds optional ingress adapters by `Arc`.
///
/// At least one transport must be configured before
/// [`RuntimeManager::start`] is called.
pub struct DefaultInput {
    pub(crate) http: Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc: Option<Arc<dyn GrpcInbound>>,
}

impl DefaultInput {
    /// Start with HTTP as the sole transport.
    pub fn new_http(http: Arc<dyn HttpInbound>) -> Self {
        Self { http: Some(http), grpc: None }
    }
    /// Start with gRPC as the sole transport.
    pub fn new_grpc(grpc: Arc<dyn GrpcInbound>) -> Self {
        Self { http: None, grpc: Some(grpc) }
    }
    /// Construct with no transports configured.
    pub fn empty() -> Self { Self { http: None, grpc: None } }
    /// Add (or replace) the HTTP transport.
    pub fn with_http(mut self, http: Arc<dyn HttpInbound>) -> Self {
        self.http = Some(http); self
    }
    /// Add (or replace) the gRPC transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcInbound>) -> Self {
        self.grpc = Some(grpc); self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: Input::has_any
    #[test]
    fn test_has_any_false_with_no_transports() {
        struct NoTransport;
        impl Input for NoTransport {
            fn http(&self) -> Option<Arc<dyn HttpInbound>> { None }
            fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> { None }
        }
        assert!(!NoTransport.has_any());
    }

    /// @covers: DefaultInput struct declaration
    #[test]
    fn test_default_input_fields_accessible_within_crate() {
        let input = DefaultInput { http: None, grpc: None };
        assert!(input.http.is_none());
        assert!(input.grpc.is_none());
    }

    /// @covers: DefaultInput::empty
    #[test]
    fn test_empty_creates_input_with_no_transports() {
        let input = DefaultInput::empty();
        assert!(input.http.is_none());
        assert!(input.grpc.is_none());
    }

    /// @covers: DefaultInput::new_http
    #[test]
    fn test_new_http_sets_http_transport() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpInboundResult, HttpHealthCheck};
        struct Stub;
        impl HttpInbound for Stub {
            fn handle(&self, _: swe_edge_ingress::HttpRequest, _: edge_domain::RequestContext)
                -> BoxFuture<'_, HttpInboundResult<swe_edge_ingress::HttpResponse>>
            { Box::pin(async { Ok(swe_edge_ingress::HttpResponse::new(200, vec![])) }) }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>>
            { Box::pin(async { Ok(HttpHealthCheck::healthy()) }) }
        }
        let input = DefaultInput::new_http(Arc::new(Stub));
        assert!(input.http.is_some());
        assert!(input.grpc.is_none());
    }

    /// @covers: DefaultInput::new_grpc
    #[test]
    fn test_new_grpc_sets_grpc_transport() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{GrpcInboundResult, GrpcHealthCheck, GrpcInboundError,
            GrpcRequest, GrpcResponse, GrpcMetadata, GrpcMessageStream};
        struct Stub;
        impl GrpcInbound for Stub {
            fn handle_unary(&self, _: GrpcRequest, _: edge_domain::RequestContext)
                -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: edge_domain::RequestContext)
                -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>>
            { Box::pin(async { Ok(GrpcHealthCheck::healthy()) }) }
        }
        let input = DefaultInput::new_grpc(Arc::new(Stub));
        assert!(input.http.is_none());
        assert!(input.grpc.is_some());
    }

    /// @covers: DefaultInput::with_http
    #[test]
    fn test_with_http_replaces_transport() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpInboundResult, HttpHealthCheck};
        struct Stub;
        impl HttpInbound for Stub {
            fn handle(&self, _: swe_edge_ingress::HttpRequest, _: edge_domain::RequestContext)
                -> BoxFuture<'_, HttpInboundResult<swe_edge_ingress::HttpResponse>>
            { Box::pin(async { Ok(swe_edge_ingress::HttpResponse::new(200, vec![])) }) }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>>
            { Box::pin(async { Ok(HttpHealthCheck::healthy()) }) }
        }
        let input = DefaultInput::empty().with_http(Arc::new(Stub));
        assert!(input.http.is_some());
    }

    /// @covers: DefaultInput::with_grpc
    #[test]
    fn test_with_grpc_replaces_transport() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{GrpcInboundResult, GrpcHealthCheck, GrpcInboundError,
            GrpcRequest, GrpcResponse, GrpcMetadata, GrpcMessageStream};
        struct Stub;
        impl GrpcInbound for Stub {
            fn handle_unary(&self, _: GrpcRequest, _: edge_domain::RequestContext)
                -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: edge_domain::RequestContext)
                -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>>
            { Box::pin(async { Ok(GrpcHealthCheck::healthy()) }) }
        }
        let input = DefaultInput::empty().with_grpc(Arc::new(Stub));
        assert!(input.grpc.is_some());
    }
}
