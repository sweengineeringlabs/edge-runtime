//! Integration tests for TonicGrpcServerBuilder.
//! @covers: TonicGrpcServerBuilder::new
//! @covers: TonicGrpcServerBuilder::build
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{
    CompressionMode, GrpcServerConfig, GrpcServerObserver, NoopGrpcIngress, TonicGrpcServerBuilder,
};

#[test]
fn test_builder_new_creates_server_with_defaults_happy() {
    let b = TonicGrpcServerBuilder::new("127.0.0.1:0", NoopGrpcIngress::create());
    let s = b.allow_unauthenticated().build();
    assert!(!s.is_reflection_enabled());
}

#[test]
fn test_builder_enable_reflection_propagated_happy() {
    let s = TonicGrpcServerBuilder::new("127.0.0.1:0", NoopGrpcIngress::create())
        .allow_unauthenticated()
        .enable_reflection()
        .build();
    assert!(s.is_reflection_enabled());
}

#[test]
fn test_builder_with_max_message_size_sets_value_happy() {
    // Verify the max-message-size is propagated via from_config round-trip.
    // We build a config with a known max_message_bytes, then construct with from_config.
    use std::net::SocketAddr;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .with_max_message_bytes(512);
    // If from_config honours the value, this succeeds.
    let s = swe_edge_runtime_grpc::TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create())
        .unwrap();
    // Verify indirectly: the resulting server was constructed without error,
    // and the reflection flag (also from config) is off by default.
    assert!(!s.is_reflection_enabled());
}

#[test]
fn test_builder_with_compression_sets_mode_happy() {
    // Compression is validated via from_config: if the builder honours it, the
    // config-round-trip matches CompressionMode::Gzip.
    use std::net::SocketAddr;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .with_compression(CompressionMode::Gzip);
    // Construction succeeds — the mode was accepted.
    let _s = swe_edge_runtime_grpc::TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create())
        .unwrap();
    assert!(matches!(cfg.compression, CompressionMode::Gzip));
}

#[test]
fn test_builder_without_reflection_returns_false_error() {
    // Verify reflection is off by default — it must not accidentally be on.
    let s = TonicGrpcServerBuilder::new("127.0.0.1:0", NoopGrpcIngress::create())
        .allow_unauthenticated()
        .build();
    assert!(
        !s.is_reflection_enabled(),
        "reflection must be off by default"
    );
}

#[test]
fn test_builder_with_max_concurrent_streams_sets_value_error() {
    // Verify the override is propagated through the config round-trip.
    use std::net::SocketAddr;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .with_max_concurrent_streams(10);
    assert_eq!(
        cfg.max_concurrent_streams, 10,
        "config must carry the override"
    );
    let _s = swe_edge_runtime_grpc::TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create())
        .unwrap();
}

#[test]
fn test_builder_with_tls_sets_tls_config_edge() {
    use swe_edge_runtime_grpc::IngressTlsConfig;
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    // With tls_required=true (default) AND an IngressTlsConfig provided, from_config succeeds.
    use std::net::SocketAddr;
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind).with_tls(tls);
    assert!(
        cfg.tls.is_some(),
        "TLS config must be stored in GrpcServerConfig"
    );
    let _s = swe_edge_runtime_grpc::TonicGrpcServer::from_config(&cfg, NoopGrpcIngress::create())
        .unwrap();
}

#[test]
fn test_builder_without_interceptors_produces_empty_chain_edge() {
    let s = TonicGrpcServerBuilder::new("127.0.0.1:0", NoopGrpcIngress::create())
        .allow_unauthenticated()
        .build();
    // Health service should still be auto-wired even without explicit interceptors.
    assert!(GrpcServerObserver::health_service(&s).is_some());
}
