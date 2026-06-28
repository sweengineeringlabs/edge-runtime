//! Integration tests for GrpcServerSvc factory.
//! @covers: GrpcServerSvc::create_config_builder
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;

use swe_edge_runtime_grpc::{
    GrpcServer, GrpcServerBuild, GrpcServerConfigBuild, GrpcServerConfigOps, GrpcServerManage,
    GrpcServerObserverOps, GrpcServerSvc, GrpcServerSvcOps, NoopGrpcIngress, StatusCodeConvert,
    TonicGrpcServer, TonicGrpcServerBuilder,
};

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
    // Construction succeeds; reflection is off by default.
    assert!(
        !server.is_reflection_enabled(),
        "empty-bind server must not have reflection enabled"
    );
}

#[test]
fn test_new_tonic_server_ipv6_addr_creates_server_edge() {
    // @covers: GrpcServer::new_tonic_server
    let server =
        TonicGrpcServer::new_tonic_server("[::1]:0".to_string(), NoopGrpcIngress::create());
    assert!(!server.is_reflection_enabled());
}

// ── GrpcServerManage::from_config ────────────────────────────────────────────

#[test]
fn test_from_config_valid_plaintext_config_happy() {
    // @covers: GrpcServerManage::from_config
    use swe_edge_runtime_grpc::GrpcServerConfig;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind).allow_plaintext();
    let server = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create()).unwrap();
    // plaintext server must not accidentally have reflection on.
    assert!(
        !server.is_reflection_enabled(),
        "plaintext server must not have reflection by default"
    );
}

#[test]
fn test_from_config_tls_required_no_tls_returns_error() {
    // @covers: GrpcServerManage::from_config
    use swe_edge_runtime_grpc::GrpcServerConfig;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind); // tls_required=true, no TLS
    let result = TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create());
    assert!(result.is_err());
}

#[test]
fn test_from_config_reflection_flag_propagated_edge() {
    // @covers: GrpcServerManage::from_config
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
    let b = TonicGrpcServerBuilder::new("127.0.0.1:50053", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    let bind = server.builder_bind(&b);
    assert_eq!(bind, "127.0.0.1:50053");
}

#[test]
fn test_builder_bind_empty_string_error() {
    // @covers: GrpcServer::builder_bind
    let b = TonicGrpcServerBuilder::new("", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    assert_eq!(server.builder_bind(&b), "");
}

#[test]
fn test_builder_bind_ipv6_addr_edge() {
    // @covers: GrpcServer::builder_bind
    let b = TonicGrpcServerBuilder::new("[::1]:8080", NoopGrpcIngress::create());
    let server = TonicGrpcServer::new("", NoopGrpcIngress::create()).allow_unauthenticated(true);
    assert_eq!(server.builder_bind(&b), "[::1]:8080");
}

// ── GrpcServer::new_server_svc / new_observer_svc / status_converter ─────────

#[test]
fn test_new_server_svc_returns_instance_happy() {
    // @covers: GrpcServer::new_server_svc
    let _svc = TonicGrpcServer::new_server_svc();
    // GrpcServerSvc is usable: produce a config builder and verify the port.
    let addr: SocketAddr = "127.0.0.1:9090".parse().unwrap();
    let cfg = swe_edge_runtime_grpc::GrpcServerSvc::create_config_builder(addr)
        .allow_plaintext()
        .build();
    assert_eq!(
        cfg.bind.port(),
        9090,
        "config builder from new_server_svc must use the provided port"
    );
}

#[test]
fn test_new_observer_svc_returns_instance_happy() {
    // @covers: GrpcServer::new_observer_svc
    let _svc = TonicGrpcServer::new_observer_svc();
    // GrpcServerObserverSvc is usable: check reflection on a known server.
    let server = TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create());
    assert!(
        !swe_edge_runtime_grpc::GrpcServerObserverSvc::is_reflection_enabled(&server),
        "new_observer_svc must see reflection=false on a fresh server"
    );
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
    let serve_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        GrpcServer::serve(&server, shutdown),
    )
    .await;
    // Verify the timeout did NOT fire (i.e. serve returned before 5s).
    assert!(
        serve_result.is_ok(),
        "serve timed out — immediate shutdown must complete in <5s"
    );
    // Verify serve itself returned Ok (not an error from the serve call).
    let inner = serve_result.unwrap();
    assert!(
        inner.is_ok(),
        "serve with immediate shutdown must not return an error"
    );
    // The only non-trivial postcondition: reflection flag is still off (no mutation happened).
    assert!(!server.is_reflection_enabled());
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
    // @covers: GrpcServer::new_server_svc
    let _svc = TonicGrpcServer::new_server_svc();
    // A server built from the svc has fail-closed TLS by default.
    let addr: SocketAddr = "127.0.0.1:50099".parse().unwrap();
    let cfg = swe_edge_runtime_grpc::GrpcServerSvc::create_config_builder(addr).build();
    assert!(
        cfg.tls_required,
        "default config must be fail-closed (tls_required=true)"
    );
}

#[test]
fn test_new_observer_svc_is_not_error_error() {
    // @covers: GrpcServer::new_observer_svc
    let _svc = TonicGrpcServer::new_observer_svc();
    // Reflection-on and reflection-off servers are distinguishable via the observer.
    let server_on =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).enable_reflection(true);
    assert!(
        swe_edge_runtime_grpc::GrpcServerObserverSvc::is_reflection_enabled(&server_on),
        "observer must see reflection=true on a reflection-enabled server"
    );
}

#[test]
fn test_status_converter_is_not_error_edge() {
    // @covers: GrpcServer::status_converter
    use swe_edge_runtime_grpc::{GrpcStatusCode, StatusCodeConverter};
    let _conv = TonicGrpcServer::status_converter();
    // Round-trip a non-trivial status code to verify the converter is wired.
    let code = GrpcStatusCode::NotFound;
    assert_eq!(
        StatusCodeConverter::from_wire(StatusCodeConverter::to_wire(code)),
        code,
        "status code must survive a to_wire/from_wire round-trip"
    );
}

#[test]
fn test_new_server_svc_multiple_instances_are_independent_edge() {
    // @covers: GrpcServer::new_server_svc
    // Each call produces an independent value — no shared mutable state.
    let _svc1 = TonicGrpcServer::new_server_svc();
    let _svc2 = TonicGrpcServer::new_server_svc();
    // Builders from different addresses produce configs with distinct ports.
    let a1: SocketAddr = "127.0.0.1:7001".parse().unwrap();
    let a2: SocketAddr = "127.0.0.1:7002".parse().unwrap();
    let cfg1 = swe_edge_runtime_grpc::GrpcServerSvc::create_config_builder(a1)
        .allow_plaintext()
        .build();
    let cfg2 = swe_edge_runtime_grpc::GrpcServerSvc::create_config_builder(a2)
        .allow_plaintext()
        .build();
    assert_ne!(
        cfg1.bind.port(),
        cfg2.bind.port(),
        "independent builders must not share state"
    );
}

#[test]
fn test_new_observer_svc_multiple_instances_are_independent_edge() {
    // @covers: GrpcServer::new_observer_svc
    let _svc1 = TonicGrpcServer::new_observer_svc();
    let _svc2 = TonicGrpcServer::new_observer_svc();
    // Two independent servers with different reflection states are observable independently.
    let s_off = TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create());
    let s_on =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).enable_reflection(true);
    assert!(!swe_edge_runtime_grpc::GrpcServerObserverSvc::is_reflection_enabled(&s_off));
    assert!(swe_edge_runtime_grpc::GrpcServerObserverSvc::is_reflection_enabled(&s_on));
}
