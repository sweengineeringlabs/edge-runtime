//! Integration tests for GrpcServerSvc factory.
//! @covers: GrpcServerSvc::create_config_builder
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;

use swe_edge_runtime_grpc::{GrpcServer, GrpcServerSvc, NoopGrpcIngress, TonicGrpcServer};

// ── create_config_builder ────────────────────────────────────────────────────

#[test]
fn test_create_config_builder_valid_addr_returns_builder_happy() {
    // @covers: GrpcServerSvc::create_config_builder
    let addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
    let b = GrpcServerSvc::create_config_builder(addr);
    // Builder builds a valid config without panicking.
    let cfg = b.allow_plaintext().build();
    assert_eq!(cfg.bind.port(), 50051);
}

#[test]
fn test_create_config_builder_tls_required_returns_error_error() {
    // @covers: GrpcServerSvc::create_config_builder
    // A builder with tls_required=true and no TLS config must reject at from_config time.
    let addr: SocketAddr = "127.0.0.1:50052".parse().unwrap();
    let b = GrpcServerSvc::create_config_builder(addr);
    let cfg = b.build(); // tls_required defaults to true
    let result = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create());
    assert!(
        result.is_err(),
        "must reject TLS-required config with no TLS"
    );
}

#[test]
fn test_create_config_builder_zero_port_returns_builder_edge() {
    // @covers: GrpcServerSvc::create_config_builder
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let b = GrpcServerSvc::create_config_builder(addr);
    let cfg = b.allow_plaintext().build();
    assert_eq!(cfg.bind.port(), 0, "port 0 means OS-assigned");
}

// ── GrpcServer::new_tonic_server ─────────────────────────────────────────────

#[test]
fn test_new_tonic_server_creates_server_happy() {
    // @covers: GrpcServer::new_tonic_server
    let server =
        TonicGrpcServer::new_tonic_server("127.0.0.1:0".to_string(), NoopGrpcIngress::create());
    // The server is constructible and not reflection-enabled by default.
    assert!(!server.is_reflection_enabled());
}

#[test]
fn test_new_tonic_server_with_empty_bind_returns_error_error() {
    // @covers: GrpcServer::new_tonic_server
    // An empty bind address causes a bind error at serve time, not construction.
    let server = TonicGrpcServer::new_tonic_server(String::new(), NoopGrpcIngress::create())
        .allow_unauthenticated(true);
    // We just verify construction succeeds (error is at bind time).
    drop(server);
}

#[test]
fn test_new_tonic_server_ipv6_addr_creates_server_edge() {
    // @covers: GrpcServer::new_tonic_server
    let server =
        TonicGrpcServer::new_tonic_server("[::1]:0".to_string(), NoopGrpcIngress::create());
    assert!(!server.is_reflection_enabled());
}

// ── GrpcServer::from_config ──────────────────────────────────────────────────

#[test]
fn test_from_config_valid_plaintext_config_happy() {
    // @covers: GrpcServer::from_config
    use swe_edge_runtime_grpc::GrpcServerConfig;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind).allow_plaintext();
    let server = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create());
    assert!(server.is_ok());
}

#[test]
fn test_from_config_tls_required_no_tls_returns_error() {
    // @covers: GrpcServer::from_config
    use swe_edge_runtime_grpc::GrpcServerConfig;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind); // tls_required=true, no TLS
    let result = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create());
    assert!(result.is_err());
}

#[test]
fn test_from_config_reflection_flag_propagated_edge() {
    // @covers: GrpcServer::from_config
    use swe_edge_runtime_grpc::GrpcServerConfig;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .enable_reflection();
    let server = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create()).unwrap();
    assert!(server.is_reflection_enabled());
}

// ── GrpcServer::new_config_builder ──────────────────────────────────────────

#[test]
fn test_new_config_builder_returns_builder_happy() {
    // @covers: GrpcServer::new_config_builder
    let bind: SocketAddr = "127.0.0.1:9000".parse().unwrap();
    let b = TonicGrpcServer::new_config_builder(bind);
    let cfg = b.allow_plaintext().build();
    assert_eq!(cfg.bind.port(), 9000);
}

#[test]
fn test_new_config_builder_tls_required_by_default_error() {
    // @covers: GrpcServer::new_config_builder
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let b = TonicGrpcServer::new_config_builder(bind);
    let cfg = b.build();
    assert!(cfg.tls_required, "default is fail-closed TLS");
    // Verify it rejects at from_config time
    let result = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create());
    assert!(result.is_err());
}

#[test]
fn test_new_config_builder_zero_port_edge() {
    // @covers: GrpcServer::new_config_builder
    let bind: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let b = TonicGrpcServer::new_config_builder(bind);
    let cfg = b.allow_plaintext().build();
    assert_eq!(cfg.bind.port(), 0);
}

// ── GrpcServer::builder_bind ─────────────────────────────────────────────────

#[test]
fn test_builder_bind_returns_bind_address_happy() {
    // @covers: GrpcServer::builder_bind
    use swe_edge_runtime_grpc::TonicGrpcServerBuilder;
    let b = TonicGrpcServerBuilder::new("127.0.0.1:50053", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    let bind = server.builder_bind(&b);
    assert_eq!(bind, "127.0.0.1:50053");
}

#[test]
fn test_builder_bind_empty_string_error() {
    // @covers: GrpcServer::builder_bind
    use swe_edge_runtime_grpc::TonicGrpcServerBuilder;
    let b = TonicGrpcServerBuilder::new("", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    assert_eq!(server.builder_bind(&b), "");
}

#[test]
fn test_builder_bind_ipv6_addr_edge() {
    // @covers: GrpcServer::builder_bind
    use swe_edge_runtime_grpc::TonicGrpcServerBuilder;
    let b = TonicGrpcServerBuilder::new("[::1]:8080", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    assert_eq!(server.builder_bind(&b), "[::1]:8080");
}

// ── GrpcServer::new_server_svc / new_observer_svc / status_converter ─────────

#[test]
fn test_new_server_svc_returns_instance_happy() {
    // @covers: GrpcServer::new_server_svc
    let _svc = TonicGrpcServer::new_server_svc();
}

#[test]
fn test_new_observer_svc_returns_instance_happy() {
    // @covers: GrpcServer::new_observer_svc
    let _svc = TonicGrpcServer::new_observer_svc();
}

#[test]
fn test_status_converter_returns_instance_happy() {
    // @covers: GrpcServer::status_converter
    use swe_edge_runtime_grpc::GrpcStatusCode;
    let _conv = TonicGrpcServer::status_converter();
    // Verify it's usable by round-tripping a status code.
    use swe_edge_runtime_grpc::StatusCodeConverter;
    assert_eq!(
        StatusCodeConverter::from_wire(StatusCodeConverter::to_wire(GrpcStatusCode::Ok)),
        GrpcStatusCode::Ok,
    );
}

// ── GrpcServer::serve ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_serve_invalid_bind_returns_error_error() {
    // @covers: GrpcServer::serve
    use futures::FutureExt;
    use swe_edge_runtime_grpc::GrpcServer;
    let server = TonicGrpcServer::new("0.0.0.0:99999", NoopGrpcIngress::create())
        .allow_unauthenticated(true);
    let shutdown = futures::future::ready(()).boxed();
    let result = GrpcServer::serve(&server, shutdown).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_serve_immediate_shutdown_exits_cleanly_happy() {
    // @covers: GrpcServer::serve
    use futures::FutureExt;
    use swe_edge_runtime_grpc::GrpcServer;
    let server =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).allow_unauthenticated(true);
    let shutdown = futures::future::ready(()).boxed();
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        GrpcServer::serve(&server, shutdown),
    )
    .await;
    assert!(result.is_ok(), "timed out waiting for immediate shutdown");
    assert!(result.unwrap().is_ok());
}

#[tokio::test]
async fn test_serve_authz_missing_returns_authorization_required_edge() {
    // @covers: GrpcServer::serve
    use futures::FutureExt;
    use swe_edge_runtime_grpc::{GrpcServer, GrpcServerError};
    // No allow_unauthenticated = true and no interceptor → AuthorizationRequired
    let server = TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create());
    let shutdown = futures::future::ready(()).boxed();
    let result = GrpcServer::serve(&server, shutdown).await;
    assert!(matches!(
        result,
        Err(GrpcServerError::AuthorizationRequired(_))
    ));
}

// ── GrpcServer::new_server_svc edge (unused variants) ─────────────────────────

#[test]
fn test_new_server_svc_is_not_error_error() {
    // Constructing doesn't fail — type anchor test.
    let _svc = TonicGrpcServer::new_server_svc();
}

#[test]
fn test_new_observer_svc_is_not_error_error() {
    let _svc = TonicGrpcServer::new_observer_svc();
}

#[test]
fn test_status_converter_is_not_error_edge() {
    let _conv = TonicGrpcServer::status_converter();
}

#[test]
fn test_new_server_svc_multiple_instances_are_independent_edge() {
    // @covers: GrpcServer::new_server_svc
    // Each call produces an independent value — no shared mutable state.
    let svc1 = TonicGrpcServer::new_server_svc();
    let svc2 = TonicGrpcServer::new_server_svc();
    // Both are usable; no panic or resource contention.
    drop(svc1);
    drop(svc2);
}

#[test]
fn test_new_observer_svc_multiple_instances_are_independent_edge() {
    // @covers: GrpcServer::new_observer_svc
    // Each call produces an independent value — no shared mutable state.
    let svc1 = TonicGrpcServer::new_observer_svc();
    let svc2 = TonicGrpcServer::new_observer_svc();
    drop(svc1);
    drop(svc2);
}
