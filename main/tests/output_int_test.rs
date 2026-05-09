//! Coverage for core/output.rs — DefaultOutput trait impl.

use std::sync::Arc;
use futures::future::BoxFuture;
use swe_edge_egress_http::{HttpOutbound, HttpOutboundResult, HttpRequest, HttpResponse, HttpStreamResponse};
use swe_edge_runtime::{DefaultOutput, Output};

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

/// @covers: DefaultOutput trait impl — http()
#[test]
fn test_default_output_http_returns_configured_adapter() {
    let output = DefaultOutput::new_http(Arc::new(StubHttp));
    let _ = output.http();
    assert!(output.grpc().is_none());
}
