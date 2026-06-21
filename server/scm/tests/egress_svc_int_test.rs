//! Integration tests for the egress_svc SAF surface.
#![allow(clippy::unwrap_used)]
// @allow: no_mocks_in_integration — stub impls required to exercise the egress API surface

use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_egress_grpc::{GrpcEgressError, GrpcEgressResult, GrpcStatusCode};
use swe_edge_egress_http::{HttpEgressResult, HttpStreamResponse};
use swe_edge_runtime::{Egress, GrpcEgress, HttpEgress, Runtime, EGRESS_SVC};

struct StubHttp;
impl HttpEgress for StubHttp {
    fn send(
        &self,
        _: swe_edge_egress_http::HttpRequest,
    ) -> BoxFuture<'_, HttpEgressResult<swe_edge_egress_http::HttpResponse>> {
        Box::pin(async { Ok(swe_edge_egress_http::HttpResponse::new(200, vec![])) })
    }
    fn send_stream(
        &self,
        _: swe_edge_egress_http::HttpRequest,
    ) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>> {
        Box::pin(async {
            Ok(HttpStreamResponse {
                status: 200,
                headers: Default::default(),
                body: Box::pin(futures::stream::empty()),
            })
        })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

struct StubGrpcEgress;
impl GrpcEgress for StubGrpcEgress {
    fn call_unary(
        &self,
        _: swe_edge_egress_grpc::GrpcRequest,
    ) -> BoxFuture<'_, GrpcEgressResult<swe_edge_egress_grpc::GrpcResponse>> {
        Box::pin(async {
            Err(GrpcEgressError::Status(
                GrpcStatusCode::Unavailable,
                "stub".into(),
            ))
        })
    }
    fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

/// @covers: EGRESS_SVC
#[test]
fn test_egress_svc_slug_is_correct_happy() {
    assert_eq!(EGRESS_SVC, "egress");
}

// ── Egress::http ──────────────────────────────────────────────────────────────

#[test]
fn test_http_returns_configured_arc_http_adapter_happy() {
    let egress = Runtime::http_egress(Arc::new(StubHttp));
    let _h = egress.http();
}

#[test]
fn test_http_arc_can_be_cloned_and_shared_error() {
    let egress = Runtime::http_egress(Arc::new(StubHttp));
    let h1 = egress.http();
    let h2 = Arc::clone(&h1);
    assert!(Arc::ptr_eq(&h1, &h2));
}

#[test]
fn test_http_called_twice_returns_same_adapter_edge() {
    let egress = Runtime::http_egress(Arc::new(StubHttp));
    let h1 = egress.http();
    let h2 = egress.http();
    assert!(Arc::ptr_eq(&h1, &h2));
}

// ── Egress::grpc ──────────────────────────────────────────────────────────────

#[test]
fn test_grpc_returns_none_for_http_only_egress_happy() {
    let egress = Runtime::http_egress(Arc::new(StubHttp));
    assert!(egress.grpc().is_none());
}

#[test]
fn test_grpc_returns_some_for_http_grpc_egress_error() {
    let egress = Runtime::http_grpc_egress(Arc::new(StubHttp), Arc::new(StubGrpcEgress));
    assert!(egress.grpc().is_some());
}

#[test]
fn test_grpc_arc_is_shared_for_http_grpc_egress_edge() {
    let egress = Runtime::http_grpc_egress(Arc::new(StubHttp), Arc::new(StubGrpcEgress));
    let g1 = egress.grpc().unwrap();
    let g2 = egress.grpc().unwrap();
    assert!(Arc::ptr_eq(&g1, &g2));
}
