//! Coverage for api/ingress — DefaultIngress trait impl.

use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_ingress::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse, RequestContext,
};
use swe_edge_runtime::{DefaultIngress, Ingress};

struct Stub;
impl HttpIngress for Stub {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

/// @covers: DefaultIngress trait impl — http()
#[test]
fn test_default_ingress_http_returns_configured_adapter() {
    let ingress = DefaultIngress::new_http(Arc::new(Stub));
    assert!(ingress.http().is_some());
    assert!(ingress.grpc().is_none());
    assert!(ingress.has_any());
}

/// @covers: DefaultIngress trait impl — has_any with no transports
#[test]
fn test_default_ingress_empty_has_any_is_false() {
    let ingress = DefaultIngress::empty();
    assert!(!ingress.has_any());
}
