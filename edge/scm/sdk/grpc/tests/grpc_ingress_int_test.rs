//! Integration tests for [`swe_edge_runtime_grpc::GrpcIngress`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::SecurityContext;
use futures::executor::block_on;
use futures::future::BoxFuture;
use swe_edge_runtime_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMethod, GrpcRequest,
    GrpcResponse, NoopGrpcIngress,
};

/// @covers: GrpcIngress::handle_unary — happy path
#[test]
fn test_handle_unary_noop_returns_ok_happy() {
    let handler = NoopGrpcIngress::create();
    let req = GrpcRequest::new("acme.Greeter", "SayHello", vec![]);
    let ctx = SecurityContext::unauthenticated();
    let result = block_on(handler.handle_unary(&req, ctx));
    assert!(result.is_ok(), "expected Ok, got: {result:?}");
}

/// @covers: GrpcIngress::handle_unary — response body is empty for NoopGrpcIngress
#[test]
fn test_handle_unary_noop_response_is_empty_error() {
    let handler = NoopGrpcIngress::create();
    let req = GrpcRequest::new("acme.Greeter", "SayHello", b"some body".to_vec());
    let ctx = SecurityContext::unauthenticated();
    let resp = block_on(handler.handle_unary(&req, ctx)).unwrap();
    assert!(
        resp.body.is_empty(),
        "NoopGrpcIngress must return empty body"
    );
}

/// @covers: GrpcIngress::handle_unary — any method name is accepted
#[test]
fn test_handle_unary_noop_any_method_accepted_edge() {
    let handler = NoopGrpcIngress::create();
    let req = GrpcRequest::new("pkg.Service", "UnknownMethod", vec![]);
    let ctx = SecurityContext::unauthenticated();
    let result = block_on(handler.handle_unary(&req, ctx));
    assert!(result.is_ok());
}

/// @covers: GrpcIngress::health_check — happy path
#[test]
fn test_health_check_noop_returns_healthy_happy() {
    let handler = NoopGrpcIngress::create();
    let result = block_on(handler.health_check());
    assert!(result.is_ok());
    assert!(result.unwrap().healthy);
}

/// @covers: GrpcIngress::health_check — called twice stays healthy
#[test]
fn test_health_check_noop_idempotent_edge() {
    let handler = NoopGrpcIngress::create();
    let _ = block_on(handler.health_check());
    let result = block_on(handler.health_check());
    assert!(result.is_ok());
}

/// @covers: GrpcIngress is object-safe
#[test]
fn test_grpc_ingress_is_object_safe() {
    fn _assert(_: &dyn GrpcIngress) {}
}

// --- health_check error coverage ---

struct FailingIngress;

impl GrpcIngress for FailingIngress {
    fn handle_unary(
        &self,
        _req: &GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Ok(GrpcResponse::empty()) })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Err(GrpcIngressError::Unavailable("probe failed".into())) })
    }
}

/// @covers: GrpcIngress::health_check — custom impl may return Err
#[test]
fn test_health_check_failing_impl_returns_err_error() {
    let handler = FailingIngress;
    let result = block_on(handler.health_check());
    assert!(
        result.is_err(),
        "expected Err from FailingIngress::health_check"
    );
}

// --- error_kind coverage ---

/// @covers: GrpcIngress::error_kind — default impl returns the expected label
#[test]
fn test_error_kind_default_returns_expected_label_happy() {
    let handler = NoopGrpcIngress::create();
    let err = GrpcIngressError::Internal("boom".into());
    assert_eq!(handler.error_kind(&err), "grpc_ingress_error");
}

struct CustomLabelIngress;

impl GrpcIngress for CustomLabelIngress {
    fn handle_unary(
        &self,
        _req: &GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Ok(GrpcResponse::empty()) })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
    }

    fn error_kind(&self, _err: &GrpcIngressError) -> &'static str {
        "custom_label"
    }
}

/// @covers: GrpcIngress::error_kind — overridden impl returns custom label
#[test]
fn test_error_kind_custom_override_returns_custom_label_error() {
    let handler = CustomLabelIngress;
    let err = GrpcIngressError::Unauthorized("caller".into());
    assert_eq!(handler.error_kind(&err), "custom_label");
}

/// @covers: GrpcIngress::error_kind — same error always returns the same label
#[test]
fn test_error_kind_same_error_consistent_across_calls_edge() {
    let handler = NoopGrpcIngress::create();
    let err = GrpcIngressError::Internal("x".into());
    let first = handler.error_kind(&err);
    let second = handler.error_kind(&err);
    assert_eq!(first, second);
}

// --- accepted_methods coverage ---

/// @covers: GrpcIngress::accepted_methods — default impl advertises Unary only
#[test]
fn test_accepted_methods_noop_returns_unary_happy() {
    let handler = NoopGrpcIngress::create();
    let methods = handler.accepted_methods();
    assert_eq!(methods, vec![GrpcMethod::Unary]);
}

struct StreamingIngress;

impl GrpcIngress for StreamingIngress {
    fn handle_unary(
        &self,
        _req: &GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Ok(GrpcResponse::empty()) })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
    }

    fn accepted_methods(&self) -> Vec<GrpcMethod> {
        vec![]
    }
}

/// @covers: GrpcIngress::accepted_methods — custom impl may return empty (no methods accepted)
#[test]
fn test_accepted_methods_custom_empty_returns_empty_error() {
    let handler = StreamingIngress;
    assert!(handler.accepted_methods().is_empty());
}

struct MultiMethodIngress;

impl GrpcIngress for MultiMethodIngress {
    fn handle_unary(
        &self,
        _req: &GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Ok(GrpcResponse::empty()) })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
    }

    fn accepted_methods(&self) -> Vec<GrpcMethod> {
        vec![
            GrpcMethod::Unary,
            GrpcMethod::ServerStream,
            GrpcMethod::BidiStream,
        ]
    }
}

/// @covers: GrpcIngress::accepted_methods — custom impl can advertise multiple streaming types
#[test]
fn test_accepted_methods_custom_multi_returns_all_edge() {
    let handler = MultiMethodIngress;
    let methods = handler.accepted_methods();
    assert_eq!(methods.len(), 3);
    assert!(methods.contains(&GrpcMethod::Unary));
    assert!(methods.contains(&GrpcMethod::BidiStream));
}
