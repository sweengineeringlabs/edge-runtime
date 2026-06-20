//! Integration tests for the ingress_svc SAF surface.
#![allow(clippy::unwrap_used)]
// @allow: no_mocks_in_integration — stub impls required to exercise the ingress API surface

use edge_domain::SecurityContext;
use futures::future::BoxFuture;
use futures::FutureExt;
use std::sync::Arc;
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
};
use swe_edge_runtime::{Ingress, Runtime, INGRESS_SVC};

struct StubHttp;
impl HttpIngress for StubHttp {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        async move { Ok(HttpHealthCheck::healthy()) }.boxed()
    }
}

struct StubGrpc;
impl GrpcIngress for StubGrpc {
    fn handle_unary(
        &self,
        _: GrpcRequest,
        _: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
    }
    fn handle_stream(
        &self,
        _: String,
        _: GrpcMetadata,
        _: GrpcMessageStream,
        _: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
        Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
    }
    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        async move { Ok(GrpcHealthCheck::healthy()) }.boxed()
    }
}

/// @covers: INGRESS_SVC
#[test]
fn test_ingress_svc_slug_is_correct_happy() {
    assert_eq!(INGRESS_SVC, "ingress");
}

// ── Ingress::http ─────────────────────────────────────────────────────────────

#[test]
fn test_http_returns_some_for_http_ingress_happy() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.http().is_some());
}

#[test]
fn test_http_returns_none_for_empty_ingress_error() {
    let ingress = Runtime::empty_ingress();
    assert!(ingress.http().is_none());
}

#[test]
fn test_http_returns_none_for_grpc_only_ingress_edge() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.http().is_none());
}

// ── Ingress::grpc ─────────────────────────────────────────────────────────────

#[test]
fn test_grpc_returns_none_for_http_ingress_happy() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.grpc().is_none());
}

#[test]
fn test_grpc_returns_some_for_grpc_ingress_error() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.grpc().is_some());
}

#[test]
fn test_grpc_returns_none_for_empty_ingress_edge() {
    let ingress = Runtime::empty_ingress();
    assert!(ingress.grpc().is_none());
}

// ── Ingress::has_any ──────────────────────────────────────────────────────────

#[test]
fn test_has_any_returns_true_for_http_ingress_happy() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.has_any());
}

#[test]
fn test_has_any_returns_false_for_empty_ingress_error() {
    let ingress = Runtime::empty_ingress();
    assert!(!ingress.has_any());
}

#[test]
fn test_has_any_returns_true_for_grpc_ingress_edge() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.has_any());
}
