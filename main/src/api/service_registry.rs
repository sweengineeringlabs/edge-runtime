//! `ServiceRegistry` — stable egress client registry handed to handlers at construction time.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

/// Holds egress clients that handlers may use to make outbound calls.
///
/// Constructed by [`EdgeRuntimeBuilder::build_registry`] and passed to
/// handler constructors at startup — not per-request.  Share it via
/// `Arc<ServiceRegistry>`.
pub struct ServiceRegistry {
    http: Arc<dyn HttpOutbound>,
    grpc: Option<Arc<dyn GrpcOutbound>>,
}

impl ServiceRegistry {
    /// Construct a registry from an HTTP egress client and an optional gRPC client.
    pub fn new(http: Arc<dyn HttpOutbound>, grpc: Option<Arc<dyn GrpcOutbound>>) -> Self {
        Self { http, grpc }
    }

    /// Return the HTTP egress client.
    pub fn http(&self) -> &Arc<dyn HttpOutbound> { &self.http }

    /// Return the gRPC egress client, if one was registered.
    pub fn grpc(&self) -> Option<&Arc<dyn GrpcOutbound>> { self.grpc.as_ref() }
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

    /// @covers: ServiceRegistry::new — stores http client, grpc is None.
    #[test]
    fn test_new_stores_http_client_and_no_grpc() {
        let reg = ServiceRegistry::new(Arc::new(StubHttp), None);
        assert!(reg.grpc().is_none());
    }

    /// @covers: ServiceRegistry::http — returns the stored client.
    #[test]
    fn test_http_returns_stored_client() {
        let reg = ServiceRegistry::new(Arc::new(StubHttp), None);
        let _ = reg.http(); // type check
    }
}
