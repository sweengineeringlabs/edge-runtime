//! Tests for `NoopHttpIngress` — the no-op ingress stub.
// @covers NoopHttpIngress
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_http::{HttpIngress, HttpRequest, NoopHttpIngress};

#[test]
fn test_noop_http_ingress_constructs_happy() {
    let ingress = NoopHttpIngress;
    assert_eq!(
        std::mem::size_of_val(&ingress),
        0,
        "NoopHttpIngress must be a zero-size type"
    );
}

#[test]
fn test_noop_http_ingress_handle_returns_200_happy() {
    let ingress = NoopHttpIngress;
    let req = HttpRequest::get("/");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert_eq!(resp.status, 200);
}

#[test]
fn test_noop_http_ingress_handle_never_errors_error() {
    let ingress = NoopHttpIngress;
    let req = HttpRequest::post("/anything");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let result = block_on(ingress.handle(req, ctx));
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().status,
        200,
        "noop handle must always return HTTP 200"
    );
}

#[test]
fn test_noop_http_ingress_handle_empty_body_edge() {
    let ingress = NoopHttpIngress;
    let req = HttpRequest::get("/");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert!(resp.body.is_empty());
}

#[test]
fn test_noop_http_ingress_health_check_healthy_happy() {
    let ingress = NoopHttpIngress;
    let result = block_on(ingress.health_check()).unwrap();
    assert!(result.healthy);
}

#[test]
fn test_noop_http_ingress_health_check_no_message_edge() {
    let ingress = NoopHttpIngress;
    let result = block_on(ingress.health_check()).unwrap();
    assert!(result.message.is_none());
}
