//! Coverage for core/input.rs — DefaultInput trait impl.

use std::sync::Arc;
use futures::future::BoxFuture;
use swe_edge_ingress::{HttpInbound, HttpInboundResult, HttpHealthCheck, HttpRequest, HttpResponse};
use swe_edge_runtime::{DefaultInput, Input};

struct Stub;
impl HttpInbound for Stub {
    fn handle(&self, _: HttpRequest) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

/// @covers: DefaultInput trait impl — http()
#[test]
fn test_default_input_http_returns_configured_adapter() {
    let input = DefaultInput::new_http(Arc::new(Stub));
    assert!(input.http().is_some());
    assert!(input.grpc().is_none());
    assert!(input.has_any());
}

/// @covers: DefaultInput trait impl — has_any with no transports
#[test]
fn test_default_input_empty_has_any_is_false() {
    let input = DefaultInput::empty();
    assert!(!input.has_any());
}
