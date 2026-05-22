//! `ServiceRegistry` — stable egress client registry handed to handlers at construction time.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

/// Holds egress clients that handlers may use to make outbound calls.
///
/// Constructed by [`RuntimeBuilder::build_registry`] and passed to
/// handler constructors at startup — not per-request.  Share it via
/// `Arc<ServiceRegistry>`.
pub struct ServiceRegistry {
    http: Arc<dyn HttpEgress>,
    grpc: Option<Arc<dyn GrpcEgress>>,
}

impl ServiceRegistry {
    /// Construct a registry from an HTTP egress client and an optional gRPC client.
    pub fn new(http: Arc<dyn HttpEgress>, grpc: Option<Arc<dyn GrpcEgress>>) -> Self {
        Self { http, grpc }
    }

    /// Return the HTTP egress client.
    pub fn http(&self) -> &Arc<dyn HttpEgress> {
        &self.http
    }

    /// Return the gRPC egress client, if one was registered.
    pub fn grpc(&self) -> Option<&Arc<dyn GrpcEgress>> {
        self.grpc.as_ref()
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

    /// @covers: grpc
    #[test]
    fn test_new_stores_http_client_and_no_grpc() {
        let reg = ServiceRegistry::new(Arc::new(StubHttp), None);
        assert!(reg.grpc().is_none());
    }

    /// @covers: http
    #[test]
    fn test_http_returns_stored_client() {
        let reg = ServiceRegistry::new(Arc::new(StubHttp), None);
        let _ = reg.http(); // type check
    }
}
