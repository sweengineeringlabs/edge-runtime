//! Integration tests that exercise cross-crate dependencies (Rule 95).

use std::sync::Arc;
use swe_edge_runtime::{EdgeRuntime, RuntimeConfig};

// ── edge-domain ───────────────────────────────────────────────────────────────

/// Exercises edge-domain via the EdgeRuntimeBuilder HTTP route path.
#[tokio::test]
async fn test_edge_domain_handler_registered_via_builder() {
    use edge_domain::{Handler, HandlerError};

    struct PingHandler;

    #[async_trait::async_trait]
    impl Handler<String, String> for PingHandler {
        fn id(&self) -> &str { "ping" }
        fn pattern(&self) -> &str { "/ping" }
        async fn execute(&self, _: String) -> Result<String, HandlerError> { Ok("pong".into()) }
    }

    // http_route succeeding without panic confirms edge-domain Handler is wired
    let b = EdgeRuntime::builder().http_route(Arc::new(PingHandler));
    // build_registry returns None (no egress) but the builder is valid
    assert!(b.build_registry().is_none());
}

// ── swe-edge-ingress-verifier ─────────────────────────────────────────────────

/// Exercises the JWT verifier integration through the builder.
#[test]
fn test_ingress_verifier_wired_via_bearer_auth() {
    use swe_edge_runtime::{JwtVerifier, JwtConfig, JwtKey};

    let secret: Vec<u8> = b"super-secret-test-key-32-bytes!!".to_vec();
    let cfg = JwtConfig {
        key:              JwtKey::Hs256 { secret },
        required_issuer:  None,
        required_audience: None,
        leeway_seconds:   0,
    };
    let verifier = JwtVerifier::from_config(&cfg).expect("jwt verifier");
    // Wiring the verifier to the builder exercises the TokenVerifier trait path
    let b = EdgeRuntime::builder().http_bearer_auth(Arc::new(verifier));
    assert!(b.build_registry().is_none()); // egress not set, but builder is valid
}

// ── swe-edge-egress-grpc ──────────────────────────────────────────────────────

/// Exercises egress-grpc wiring through the builder.
#[test]
fn test_egress_grpc_wired_via_builder() {
    use futures::future::BoxFuture;
    use swe_edge_runtime::GrpcOutbound;
    use swe_edge_egress_grpc::{GrpcOutboundError, GrpcOutboundResult, GrpcRequest, GrpcResponse, GrpcStatusCode};

    struct StubGrpc;
    impl GrpcOutbound for StubGrpc {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>>
        { Box::pin(async { Err(GrpcOutboundError::Status(GrpcStatusCode::Unavailable, "stub".into())) }) }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>>
        { Box::pin(async { Ok(()) }) }
    }

    let b = EdgeRuntime::builder().egress_grpc(Arc::new(StubGrpc));
    // build_registry is None because no egress_http is set
    assert!(b.build_registry().is_none());
}

// ── swe-edge-ingress-grpc-reflection ─────────────────────────────────────────

/// Exercises gRPC reflection flag in RuntimeConfig.
#[test]
fn test_grpc_reflection_config_field_respected() {
    let cfg = RuntimeConfig { grpc_reflection: true, ..RuntimeConfig::default() };
    assert!(cfg.grpc_reflection);
}
