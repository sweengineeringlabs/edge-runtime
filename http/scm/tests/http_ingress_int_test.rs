//! Integration tests for the `HttpIngress` contract and `NoopHttpIngress`.
// @covers NoopHttpIngress::handle
// @covers NoopHttpIngress::health_check
// @covers NoopHttpIngress::accepted_methods
// @covers NoopHttpIngress::request_builder
// @covers NoopHttpIngress::extract_auth
// @covers NoopHttpIngress::extract_body
// @covers NoopHttpIngress::extract_form_parts
// @covers NoopHttpIngress::wrap_decode_fn
// @covers NoopHttpIngress::wrap_encode_fn
// @covers NoopHttpIngress::error_kind
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_http::{
    HttpIngress, HttpIngressError, HttpMethod, HttpRequest, HttpResponse, NoopHttpIngress,
};

fn noop() -> NoopHttpIngress {
    NoopHttpIngress
}

// ─── handle ─────────────────────────────────────────────────────────────────

#[test]
fn test_handle_noop_get_happy() {
    let ingress = noop();
    let req = HttpRequest::get("/");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert_eq!(resp.status, 200);
    assert!(resp.body.is_empty());
}

#[test]
fn test_handle_noop_post_happy() {
    let ingress = noop();
    let req = HttpRequest::post("/echo");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert!(resp.is_success());
}

#[test]
fn test_handle_noop_error() {
    // NoopHttpIngress never returns an error — verify Result is Ok (i.e. error variant is inaccessible).
    let ingress = noop();
    let req = HttpRequest::delete("/resource/1");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let result = block_on(ingress.handle(req, ctx));
    // The error path is unreachable for Noop; confirm the happy result is success.
    assert!(result.is_ok());
}

#[test]
fn test_handle_noop_put_edge() {
    // Edge: PUT to an unusual path — Noop still responds 200.
    let ingress = noop();
    let req = HttpRequest::put("/");
    let ctx = edge_domain::SecurityContext::unauthenticated();
    let resp = block_on(ingress.handle(req, ctx)).unwrap();
    assert_eq!(resp.status, 200);
}

// ─── health_check ───────────────────────────────────────────────────────────

#[test]
fn test_health_check_noop_happy() {
    let ingress = noop();
    let result = block_on(ingress.health_check()).unwrap();
    assert!(result.healthy);
    assert!(result.message.is_none());
}

#[test]
fn test_health_check_noop_error() {
    // Noop never errors; confirm the Result is Ok (documents error path is absent).
    let ingress = noop();
    let result = block_on(ingress.health_check());
    assert!(result.is_ok());
}

#[test]
fn test_health_check_noop_called_twice_edge() {
    // Edge: calling health_check multiple times is idempotent.
    let ingress = noop();
    let r1 = block_on(ingress.health_check()).unwrap();
    let r2 = block_on(ingress.health_check()).unwrap();
    assert_eq!(r1.healthy, r2.healthy);
}

// ─── accepted_methods ───────────────────────────────────────────────────────

#[test]
fn test_accepted_methods_noop_happy() {
    // Default impl returns all methods as empty — documents the default contract.
    let ingress = noop();
    let methods = ingress.accepted_methods();
    // The default returns an empty vec; a real implementation would list its methods.
    assert!(methods.is_empty());
}

#[test]
fn test_accepted_methods_noop_error() {
    // No error path; the method returns a Vec, not a Result.
    // Confirm the return type is a Vec (documents the non-fallible contract).
    let ingress = noop();
    let methods: Vec<HttpMethod> = ingress.accepted_methods();
    let _ = methods; // used
}

#[test]
fn test_accepted_methods_noop_edge() {
    // Edge: calling twice returns the same (empty) list — no side effects.
    let ingress = noop();
    assert_eq!(
        ingress.accepted_methods().len(),
        ingress.accepted_methods().len()
    );
}

// ─── request_builder ────────────────────────────────────────────────────────

#[test]
fn test_request_builder_noop_happy() {
    let ingress = noop();
    let req = ingress.request_builder().build();
    assert_eq!(req.method, HttpMethod::Get);
    assert_eq!(req.url, "/");
}

#[test]
fn test_request_builder_noop_error() {
    // Builder is non-fallible; documents that no error path exists.
    let ingress = noop();
    let builder = ingress.request_builder();
    let req = builder.build();
    assert!(!req.url.is_empty());
}

#[test]
fn test_request_builder_noop_edge() {
    // Edge: builder can be extended before build().
    let ingress = noop();
    let req = ingress
        .request_builder()
        .with_header("X-Request-ID", "abc123")
        .build();
    assert_eq!(req.header("X-Request-ID"), Some("abc123"));
}

// ─── extract_auth ───────────────────────────────────────────────────────────

#[test]
fn test_extract_auth_noop_happy() {
    // Default extract_auth always returns None (no auth extracted).
    let ingress = noop();
    let req = HttpRequest::get("/");
    assert!(ingress.extract_auth(&req).is_none());
}

#[test]
fn test_extract_auth_noop_error() {
    // Error path: request has no auth header — returns None (documents expected absence).
    let ingress = noop();
    let req = HttpRequest::get("/secure");
    let auth = ingress.extract_auth(&req);
    assert!(auth.is_none(), "Noop should return None for any request");
}

#[test]
fn test_extract_auth_noop_with_bearer_header_edge() {
    // Edge: even with an Authorization header, default impl returns None.
    let ingress = noop();
    let req = HttpRequest::get("/").with_header("Authorization", "Bearer tok123");
    assert!(ingress.extract_auth(&req).is_none());
}

// ─── extract_body ───────────────────────────────────────────────────────────

#[test]
fn test_extract_body_noop_with_body_happy() {
    let ingress = noop();
    let req = HttpRequest::get("/").with_body(b"hello".to_vec(), "text/plain");
    assert!(ingress.extract_body(&req).is_some());
}

#[test]
fn test_extract_body_noop_no_body_error() {
    // Error path: request without body returns None.
    let ingress = noop();
    let req = HttpRequest::get("/");
    assert!(ingress.extract_body(&req).is_none());
}

#[test]
fn test_extract_body_noop_empty_bytes_edge() {
    // Edge: empty byte slice body — body is Some but empty.
    let ingress = noop();
    let req = HttpRequest::get("/").with_body(vec![], "application/octet-stream");
    let body = ingress.extract_body(&req);
    assert!(body.is_some());
}

// ─── extract_form_parts ─────────────────────────────────────────────────────

#[test]
fn test_extract_form_parts_noop_happy() {
    // Default impl returns empty vec.
    let ingress = noop();
    let req = HttpRequest::post("/upload");
    let parts = ingress.extract_form_parts(&req);
    assert!(parts.is_empty());
}

#[test]
fn test_extract_form_parts_noop_error() {
    // No error path; documents the non-fallible contract.
    let ingress = noop();
    let req = HttpRequest::post("/upload");
    let parts = ingress.extract_form_parts(&req);
    // Confirm it does not panic and returns a Vec.
    let _ = parts;
}

#[test]
fn test_extract_form_parts_noop_edge() {
    // Edge: calling twice is idempotent.
    let ingress = noop();
    let req = HttpRequest::post("/upload");
    let p1 = ingress.extract_form_parts(&req);
    let p2 = ingress.extract_form_parts(&req);
    assert_eq!(p1.len(), p2.len());
}

// ─── wrap_decode_fn ─────────────────────────────────────────────────────────

#[test]
fn test_wrap_decode_fn_noop_happy() {
    // Default impl is identity — returns the same fn pointer.
    let ingress = noop();
    fn decode(req: &HttpRequest) -> Result<String, HttpIngressError> {
        Ok(req.url.clone())
    }
    let wrapped = ingress.wrap_decode_fn(decode);
    let req = HttpRequest::get("/hello");
    let result = wrapped(&req).unwrap();
    assert_eq!(result, "/hello");
}

#[test]
fn test_wrap_decode_fn_noop_error() {
    // When the wrapped fn returns an error, wrap_decode_fn passes it through unchanged.
    let ingress = noop();
    fn decode(_req: &HttpRequest) -> Result<String, HttpIngressError> {
        Err(HttpIngressError::InvalidInput("bad input".to_string()))
    }
    let wrapped = ingress.wrap_decode_fn(decode);
    let req = HttpRequest::get("/");
    let result = wrapped(&req);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HttpIngressError::InvalidInput(_)
    ));
}

#[test]
fn test_wrap_decode_fn_noop_edge() {
    // Edge: decode fn that always produces the same output — confirming identity wrap.
    let ingress = noop();
    fn decode(_req: &HttpRequest) -> Result<u32, HttpIngressError> {
        Ok(42)
    }
    let wrapped = ingress.wrap_decode_fn(decode);
    let req = HttpRequest::get("/");
    assert_eq!(wrapped(&req).unwrap(), 42);
}

// ─── wrap_encode_fn ─────────────────────────────────────────────────────────

#[test]
fn test_wrap_encode_fn_noop_happy() {
    // Default impl is identity — wrapped fn produces same output.
    let ingress = noop();
    fn encode(body: String) -> HttpResponse {
        HttpResponse::new(200, body.into_bytes())
    }
    let wrapped = ingress.wrap_encode_fn(encode);
    let resp = wrapped("hello".to_string());
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, b"hello");
}

#[test]
fn test_wrap_encode_fn_noop_error() {
    // Encode fn produces a 500 — wrap_encode_fn passes it through unchanged.
    let ingress = noop();
    fn encode(_body: ()) -> HttpResponse {
        HttpResponse::new(500, b"err".to_vec())
    }
    let wrapped = ingress.wrap_encode_fn(encode);
    let resp = wrapped(());
    assert_eq!(resp.status, 500);
}

#[test]
fn test_wrap_encode_fn_noop_edge() {
    // Edge: encode fn with unit output — confirms generic works for zero-size types.
    let ingress = noop();
    fn encode(_: u8) -> HttpResponse {
        HttpResponse::new(204, vec![])
    }
    let wrapped = ingress.wrap_encode_fn(encode);
    let resp = wrapped(0u8);
    assert_eq!(resp.status, 204);
}

// ─── error_kind ─────────────────────────────────────────────────────────────

#[test]
fn test_error_kind_noop_happy() {
    let ingress = noop();
    let err = HttpIngressError::Internal("test".to_string());
    assert_eq!(ingress.error_kind(&err), "ingress_error");
}

#[test]
fn test_error_kind_noop_not_found_error() {
    let ingress = noop();
    let err = HttpIngressError::NotFound("missing".to_string());
    // Default impl returns the same label regardless of variant.
    assert_eq!(ingress.error_kind(&err), "ingress_error");
}

#[test]
fn test_error_kind_noop_unauthorized_edge() {
    let ingress = noop();
    let err = HttpIngressError::Unauthorized("no token".to_string());
    // Edge: authentication errors still return the generic label from Noop.
    assert_eq!(ingress.error_kind(&err), "ingress_error");
}
