//! Coverage for Ingress trait impl via Runtime ingress factory methods.

use edge_domain::SecurityContext;
use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
};
use swe_edge_runtime::{Ingress, Runtime};

struct Stub;
impl HttpIngress for Stub {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

/// @covers: Runtime::http_ingress — http()
#[test]
fn test_http_ingress_returns_configured_adapter() {
    let ingress = Runtime::http_ingress(Arc::new(Stub));
    assert!(ingress.http().is_some());
    assert!(ingress.grpc().is_none());
    assert!(ingress.has_any());
}

/// @covers: Runtime::empty_ingress — has_any with no transports
#[test]
fn test_empty_ingress_has_any_is_false() {
    let ingress = Runtime::empty_ingress();
    assert!(!ingress.has_any());
}
