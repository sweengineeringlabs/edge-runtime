//! Integration tests for GrpcServerConfigBuilder.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;

use swe_edge_runtime_grpc::{CompressionMode, GrpcServerConfigBuilder, DEFAULT_MAX_MESSAGE_BYTES};

fn bind() -> SocketAddr {
    "127.0.0.1:0".parse().unwrap()
}

#[test]
fn test_grpc_server_config_builder_builds_with_defaults() {
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(cfg.tls_required);
    assert_eq!(cfg.max_message_bytes, DEFAULT_MAX_MESSAGE_BYTES);
}

#[test]
fn test_grpc_server_config_builder_allow_plaintext_clears_tls_required() {
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .build();
    assert!(!cfg.tls_required);
}

#[test]
fn test_grpc_server_config_builder_with_compression_sets_mode() {
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .with_compression(CompressionMode::Gzip)
        .build();
    assert!(matches!(cfg.compression, CompressionMode::Gzip));
}

#[test]
fn test_grpc_server_config_builder_enable_reflection_sets_flag() {
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .enable_reflection()
        .build();
    assert!(cfg.enable_reflection);
}
