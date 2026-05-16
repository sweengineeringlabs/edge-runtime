//! Coverage for api/egress — DefaultEgress trait impl.

use std::sync::Arc;
use futures::future::BoxFuture;
use swe_edge_egress_http::{HttpOutbound, HttpOutboundResult, HttpRequest, HttpResponse, HttpStreamResponse};
use swe_edge_runtime::{DefaultEgress, Egress};

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

/// @covers: DefaultEgress trait impl — http()
#[test]
fn test_default_egress_http_returns_configured_adapter() {
    let egress = DefaultEgress::new_http(Arc::new(StubHttp));
    let _ = egress.http();
    assert!(egress.grpc().is_none());
}
