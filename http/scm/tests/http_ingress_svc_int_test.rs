//! Integration tests for `saf::http_ingress_svc` — the SAF factory surface.
// @covers NoopHttpIngress::create
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_http::{HttpIngress, HttpRequest, NoopHttpIngress};

#[test]
fn test_create_noop_http_ingress_returns_200_happy() {
    let ingress = NoopHttpIngress::create();
    let req = HttpRequest::get("/");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert_eq!(resp.status, 200);
}

#[test]
fn test_create_noop_http_ingress_never_errors_error() {
    // create() itself is infallible; the resulting handler never errors on Noop.
    let ingress = NoopHttpIngress::create();
    let req = HttpRequest::delete("/x");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let result = block_on(ingress.handle(req, ctx));
    assert!(result.is_ok());
}

#[test]
fn test_create_noop_http_ingress_independent_instances_edge() {
    // Edge: create() can be called multiple times, each returns an independent instance.
    let i1 = NoopHttpIngress::create();
    let i2 = NoopHttpIngress::create();
    let r1 = block_on(i1.handle(HttpRequest::get("/ping"), edge_domain::SecurityContext::unauthenticated())).unwrap();
    let r2 = block_on(i2.handle(HttpRequest::get("/ping"), edge_domain::SecurityContext::unauthenticated())).unwrap();
    assert_eq!(r1.status, r2.status);
}
