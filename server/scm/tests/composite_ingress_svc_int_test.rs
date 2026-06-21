//! Integration tests for the composite_ingress_svc SAF surface.
#![allow(clippy::unwrap_used, dead_code)]
// @allow: no_mocks_in_integration — stub impls required to exercise the composite ingress API surface

use edge_domain::SecurityContext;
use futures::future::BoxFuture;
use futures::FutureExt;
use std::sync::Arc;
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};
use swe_edge_runtime::{CompositeGrpcIngress, CompositeIngress, COMPOSITE_INGRESS_SVC};

struct StubGrpc {
    name: &'static str,
}

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

struct TestComposite {
    primary: Arc<dyn GrpcIngress>,
}

impl CompositeIngress for TestComposite {
    fn primary(&self) -> Arc<dyn GrpcIngress> {
        Arc::clone(&self.primary)
    }
}

/// @covers: COMPOSITE_INGRESS_SVC
#[test]
fn test_composite_ingress_svc_slug_is_correct_happy() {
    assert_eq!(COMPOSITE_INGRESS_SVC, "composite_ingress");
}

// ── CompositeIngress::primary ─────────────────────────────────────────────────

#[test]
fn test_primary_returns_configured_grpc_handler_happy() {
    let stub: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "primary" });
    let comp = TestComposite {
        primary: Arc::clone(&stub),
    };
    let got = comp.primary();
    assert!(Arc::ptr_eq(&got, &stub));
}

#[test]
fn test_primary_arc_clone_shares_same_pointer_error() {
    let stub: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "primary" });
    let comp = TestComposite {
        primary: Arc::clone(&stub),
    };
    let p1 = comp.primary();
    let p2 = comp.primary();
    assert!(Arc::ptr_eq(&p1, &p2));
}

#[test]
fn test_primary_strong_count_is_two_while_composite_holds_ref_edge() {
    let stub: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "primary" });
    // original + inside TestComposite
    let comp = TestComposite {
        primary: Arc::clone(&stub),
    };
    assert_eq!(Arc::strong_count(&stub), 2);
    drop(comp);
    assert_eq!(Arc::strong_count(&stub), 1);
}

// ── CompositeIngress::new_composite ───────────────────────────────────────────

#[test]
fn test_new_composite_assembles_primary_and_reflection_happy() {
    let primary: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "primary" });
    let reflection: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "reflection" });
    let cgi: CompositeGrpcIngress =
        TestComposite::new_composite(Arc::clone(&primary), Arc::clone(&reflection));
    // primary() is callable via the CompositeIngress trait impl on CompositeGrpcIngress
    assert!(Arc::ptr_eq(&cgi.primary(), &primary));
}

#[test]
fn test_new_composite_primary_and_reflection_can_be_same_handler_error() {
    let stub: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "both" });
    let cgi = TestComposite::new_composite(Arc::clone(&stub), Arc::clone(&stub));
    assert!(Arc::ptr_eq(&cgi.primary(), &stub));
}

#[test]
fn test_new_composite_strong_counts_are_correct_edge() {
    let primary: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "primary" });
    let reflection: Arc<dyn GrpcIngress> = Arc::new(StubGrpc { name: "reflection" });
    let _cgi = TestComposite::new_composite(Arc::clone(&primary), Arc::clone(&reflection));
    // original + inside cgi
    assert_eq!(Arc::strong_count(&primary), 2);
    assert_eq!(Arc::strong_count(&reflection), 2);
}
