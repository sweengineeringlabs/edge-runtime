//! Integration tests verifying `edge-domain` dependency usage through the HTTP ingress contract.
// @covers edge_domain::SecurityContext
#![allow(clippy::unwrap_used)]

use edge_domain::SecurityContext;
use futures::executor::block_on;
use swe_edge_runtime_http::{HttpIngress, HttpRequest, NoopHttpIngress};

#[test]
fn test_edge_domain_security_context_unauthenticated_happy() {
    // Exercises edge-domain::SecurityContext used in handle().
    let ctx = SecurityContext::unauthenticated();
    let ingress = NoopHttpIngress;
    let req = HttpRequest::get("/");
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert_eq!(resp.status, 200);
}

#[test]
fn test_edge_domain_security_context_passed_through_error() {
    // Even an unauthenticated context — which would be an "error" for real auth — works with Noop.
    let ctx = SecurityContext::unauthenticated();
    let ingress = NoopHttpIngress;
    let req = HttpRequest::post("/secure");
    let result = block_on(ingress.handle(req, ctx));
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().status,
        200,
        "noop must return 200 regardless of auth context"
    );
}

#[test]
fn test_edge_domain_security_context_used_with_delete_edge() {
    // Edge: SecurityContext with a DELETE request — verifies context can be consumed multiple types.
    let ctx = SecurityContext::unauthenticated();
    let ingress = NoopHttpIngress;
    let req = HttpRequest::delete("/resource/42");
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert!(resp.is_success());
}
