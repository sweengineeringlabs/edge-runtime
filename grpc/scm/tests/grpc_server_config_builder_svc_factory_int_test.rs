//! Integration tests for GrpcServerConfigBuilder factory methods.
#![allow(clippy::unwrap_used)]

use std::net::SocketAddr;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;
use swe_edge_runtime_grpc::GrpcServerConfigBuilder;

fn bind() -> SocketAddr {
    "127.0.0.1:50052".parse().unwrap()
}

// ── new ─────────────────────────────────────────────────────────────────────

#[test]
fn test_new_sets_bind_and_requires_tls_happy() {
    // @covers: new
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert_eq!(cfg.bind.port(), 50052);
    assert!(cfg.tls_required, "builder must require TLS by default");
}

#[test]
fn test_new_default_no_tls_attached_error() {
    // @covers: new
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(
        cfg.tls.is_none(),
        "new builder must produce config with no TLS attached"
    );
}

// ── allow_plaintext ─────────────────────────────────────────────────────────

#[test]
fn test_allow_plaintext_clears_tls_required_happy() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .build();
    assert!(!cfg.tls_required, "allow_plaintext must clear tls_required");
}

#[test]
fn test_allow_plaintext_without_call_is_tls_required_error() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(
        cfg.tls_required,
        "without allow_plaintext, TLS must be required"
    );
}

#[test]
fn test_allow_plaintext_called_twice_stays_plaintext_edge() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .allow_plaintext()
        .build();
    assert!(
        !cfg.tls_required,
        "double allow_plaintext must not re-enable TLS"
    );
}

// ── with_tls ────────────────────────────────────────────────────────────────

#[test]
fn test_with_tls_stores_config_happy() {
    // @covers: with_tls
    let tls = IngressTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        client_ca_pem_path: None,
    };
    let cfg = GrpcServerConfigBuilder::new(bind()).with_tls(tls).build();
    assert!(cfg.tls.is_some(), "with_tls must store the TLS config");
    assert!(cfg.tls_required, "with_tls does not clear tls_required");
}

#[test]
fn test_with_tls_default_has_none_error() {
    // @covers: with_tls
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(cfg.tls.is_none());
}

#[test]
fn test_with_tls_overwrites_previous_config_edge() {
    // @covers: with_tls
    let tls1 = IngressTlsConfig {
        cert_pem_path: "cert1.pem".into(),
        key_pem_path: "key1.pem".into(),
        client_ca_pem_path: None,
    };
    let tls2 = IngressTlsConfig {
        cert_pem_path: "cert2.pem".into(),
        key_pem_path: "key2.pem".into(),
        client_ca_pem_path: None,
    };
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_tls(tls1)
        .with_tls(tls2)
        .build();
    assert!(
        cfg.tls.is_some(),
        "second with_tls must overwrite the first"
    );
    assert!(cfg.tls_required, "with_tls must not clear tls_required");
}

// ── with_max_message_bytes ──────────────────────────────────────────────────

#[test]
fn test_with_max_message_bytes_overrides_default_happy() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_max_message_bytes(256)
        .build();
    assert_eq!(cfg.max_message_bytes, 256);
}

#[test]
fn test_with_max_message_bytes_default_is_nonzero_error() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert_ne!(
        cfg.max_message_bytes, 0,
        "default max_message_bytes must not be zero"
    );
}

#[test]
fn test_with_max_message_bytes_value_of_one_edge() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_max_message_bytes(1)
        .build();
    assert_eq!(cfg.max_message_bytes, 1, "must accept 1 byte as minimum");
}

// ── with_max_concurrent_streams ─────────────────────────────────────────────

#[test]
fn test_with_max_concurrent_streams_overrides_default_happy() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_max_concurrent_streams(4)
        .build();
    assert_eq!(cfg.max_concurrent_streams, 4);
}

#[test]
fn test_with_max_concurrent_streams_default_is_nonzero_error() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert_ne!(
        cfg.max_concurrent_streams, 0,
        "default max_concurrent_streams must not be zero"
    );
}

#[test]
fn test_with_max_concurrent_streams_value_of_one_edge() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_max_concurrent_streams(1)
        .build();
    assert_eq!(cfg.max_concurrent_streams, 1, "must accept 1 as minimum");
    assert_ne!(cfg.max_concurrent_streams, 0);
}

// ── allow_unauthenticated ───────────────────────────────────────────────────

#[test]
fn test_allow_unauthenticated_sets_flag_happy() {
    // @covers: allow_unauthenticated
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_unauthenticated()
        .build();
    assert!(
        cfg.allow_unauthenticated,
        "allow_unauthenticated must set the flag"
    );
}

#[test]
fn test_allow_unauthenticated_default_is_false_error() {
    // @covers: allow_unauthenticated
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(
        !cfg.allow_unauthenticated,
        "default must not allow unauthenticated"
    );
}

#[test]
fn test_allow_unauthenticated_combined_with_tls_edge() {
    // @covers: allow_unauthenticated
    let tls = IngressTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        client_ca_pem_path: None,
    };
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_tls(tls)
        .allow_unauthenticated()
        .build();
    assert!(
        cfg.allow_unauthenticated,
        "allow_unauthenticated must work alongside TLS"
    );
}

// ── with_compression ────────────────────────────────────────────────────────

#[test]
fn test_with_compression_sets_gzip_happy() {
    // @covers: with_compression
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_compression(CompressionMode::Gzip)
        .build();
    assert!(matches!(cfg.compression, CompressionMode::Gzip));
}

#[test]
fn test_with_compression_default_is_none_error() {
    // @covers: with_compression
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(
        matches!(cfg.compression, CompressionMode::None),
        "default compression must be None"
    );
}

#[test]
fn test_with_compression_override_gzip_to_none_edge() {
    // @covers: with_compression
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None)
        .build();
    assert!(
        matches!(cfg.compression, CompressionMode::None),
        "compression must be overridable back to None"
    );
}

// ── enable_reflection ───────────────────────────────────────────────────────

#[test]
fn test_enable_reflection_sets_flag_happy() {
    // @covers: enable_reflection
    let cfg = GrpcServerConfigBuilder::new(bind())
        .enable_reflection()
        .build();
    assert!(cfg.enable_reflection, "enable_reflection must set the flag");
}

#[test]
fn test_enable_reflection_default_is_false_error() {
    // @covers: enable_reflection
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(!cfg.enable_reflection, "reflection must be off by default");
}

#[test]
fn test_enable_reflection_combined_with_tls_edge() {
    // @covers: enable_reflection
    let tls = IngressTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        client_ca_pem_path: None,
    };
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_tls(tls)
        .enable_reflection()
        .build();
    assert!(
        cfg.enable_reflection,
        "enable_reflection must work alongside TLS"
    );
    assert!(
        cfg.tls.is_some(),
        "TLS must survive chaining with enable_reflection"
    );
}

// ── with_keepalive ──────────────────────────────────────────────────────────

#[test]
fn test_with_keepalive_nonzero_sets_interval_happy() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_keepalive(20, 4)
        .build();
    assert_eq!(cfg.keepalive_interval_secs, Some(20));
    assert_eq!(cfg.keepalive_timeout_secs, 4);
}

#[test]
fn test_with_keepalive_zero_disables_interval_edge() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_keepalive(0, 5)
        .build();
    assert_eq!(
        cfg.keepalive_interval_secs, None,
        "interval=0 must disable keepalive"
    );
}

#[test]
fn test_with_keepalive_large_values_accepted_error() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind())
        .with_keepalive(3600, 60)
        .build();
    assert_eq!(cfg.keepalive_interval_secs, Some(3600));
    assert_ne!(
        cfg.keepalive_interval_secs,
        Some(0),
        "3600s must not be treated as disabled"
    );
}

// ── without_keepalive ───────────────────────────────────────────────────────

#[test]
fn test_without_keepalive_clears_interval_happy() {
    // @covers: without_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind())
        .without_keepalive()
        .build();
    assert!(
        cfg.keepalive_interval_secs.is_none(),
        "without_keepalive must clear the interval"
    );
}

#[test]
fn test_without_keepalive_default_has_interval_error() {
    // @covers: without_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(
        cfg.keepalive_interval_secs.is_some(),
        "default must have keepalive enabled"
    );
    assert_ne!(
        cfg.keepalive_interval_secs,
        Some(0),
        "default keepalive interval must not be zero"
    );
}

#[test]
fn test_without_keepalive_called_twice_stays_disabled_edge() {
    // @covers: without_keepalive
    let cfg = GrpcServerConfigBuilder::new(bind())
        .without_keepalive()
        .without_keepalive()
        .build();
    assert!(
        cfg.keepalive_interval_secs.is_none(),
        "double without_keepalive must remain disabled"
    );
}

// ── build ───────────────────────────────────────────────────────────────────

#[test]
fn test_build_produces_config_with_correct_bind_happy() {
    // @covers: build
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .build();
    assert_eq!(cfg.bind.port(), 50052);
    assert!(!cfg.tls_required, "build must propagate allow_plaintext");
}

#[test]
fn test_build_default_config_requires_tls_error() {
    // @covers: build
    let cfg = GrpcServerConfigBuilder::new(bind()).build();
    assert!(cfg.tls_required, "default build must require TLS");
}

#[test]
fn test_build_max_message_bytes_propagated_edge() {
    // @covers: build
    let cfg = GrpcServerConfigBuilder::new(bind())
        .allow_plaintext()
        .with_max_message_bytes(1)
        .build();
    assert_eq!(
        cfg.max_message_bytes, 1,
        "build must propagate minimum message bytes"
    );
    assert_ne!(cfg.max_message_bytes, 0);
}
