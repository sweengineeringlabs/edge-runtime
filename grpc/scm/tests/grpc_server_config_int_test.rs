//! Integration tests for GrpcServerConfig and TonicGrpcServer::from_config.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;
use std::sync::Arc;

use swe_edge_runtime_grpc::{
    GrpcServerConfig, GrpcServerConfigError, NoopGrpcIngress, TonicGrpcServer,
};

fn make_handler() -> Arc<NoopGrpcIngress> {
    NoopGrpcIngress::create()
}

#[test]
fn test_grpc_server_config_default_requires_tls() {
    let cfg = GrpcServerConfig::default();
    assert!(cfg.tls_required, "TLS-by-default invariant must hold");
}

#[test]
fn test_grpc_server_config_from_config_rejects_tls_required_without_tls() {
    let cfg = GrpcServerConfig::default();
    match TonicGrpcServer::from_config(&cfg, make_handler()) {
        Err(GrpcServerConfigError::TlsRequiredButMissing) => {}
        Ok(_) => panic!("must reject tls_required=true with tls=None"),
    }
}

#[test]
fn test_grpc_server_config_from_config_accepts_plaintext_opt_in() {
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind).allow_plaintext();
    let server = TonicGrpcServer::from_config(&cfg, make_handler()).unwrap();
    // Verify the resulting server has the expected state: no reflection, no TLS.
    assert!(!server.is_reflection_enabled(), "plaintext server must not have reflection enabled");
}

#[test]
fn test_grpc_server_config_new_sets_tls_required_true() {
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind);
    assert!(cfg.tls_required);
}

#[test]
fn test_grpc_server_config_with_max_message_bytes_overrides_default() {
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .with_max_message_bytes(1024);
    assert_eq!(cfg.max_message_bytes, 1024);
}

#[test]
fn test_grpc_server_config_enable_reflection_flips_flag() {
    let bind: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind)
        .allow_plaintext()
        .enable_reflection();
    assert!(cfg.enable_reflection);
}
