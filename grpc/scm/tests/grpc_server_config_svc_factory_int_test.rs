//! Integration tests for GrpcServerConfig factory methods.
#![allow(clippy::unwrap_used)]

use std::net::SocketAddr;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;
use swe_edge_runtime_grpc::{GrpcServerConfig, GrpcServerConfigOps};

fn bind() -> SocketAddr {
    "127.0.0.1:50051".parse().unwrap()
}

// ── default_keepalive_interval_secs ─────────────────────────────────────────

#[test]
fn test_default_keepalive_interval_secs_returns_some_happy() {
    // @covers: default_keepalive_interval_secs
    let v = GrpcServerConfig::default_keepalive_interval_secs();
    assert!(v.is_some(), "default must return Some interval");
    assert_ne!(v, Some(0), "default interval must not be zero");
}

#[test]
fn test_default_keepalive_interval_secs_is_not_none_error() {
    // @covers: default_keepalive_interval_secs
    let v = GrpcServerConfig::default_keepalive_interval_secs();
    assert_ne!(
        v, None,
        "keepalive must be enabled by default — None disables it"
    );
}

#[test]
fn test_default_keepalive_interval_secs_within_reasonable_range_edge() {
    // @covers: default_keepalive_interval_secs
    let v = GrpcServerConfig::default_keepalive_interval_secs().unwrap();
    assert!(
        v > 0 && v <= 3600,
        "default keepalive interval must be positive and within 1 hour"
    );
}

// ── default_keepalive_timeout_secs ──────────────────────────────────────────

#[test]
fn test_default_keepalive_timeout_secs_returns_nonzero_happy() {
    // @covers: default_keepalive_timeout_secs
    let v = GrpcServerConfig::default_keepalive_timeout_secs();
    assert_ne!(v, 0, "default timeout must not be zero");
}

#[test]
fn test_default_keepalive_timeout_secs_is_not_zero_error() {
    // @covers: default_keepalive_timeout_secs
    assert_ne!(
        GrpcServerConfig::default_keepalive_timeout_secs(),
        0,
        "timeout 0 means no timeout"
    );
}

#[test]
fn test_default_keepalive_timeout_secs_less_than_interval_edge() {
    // @covers: default_keepalive_timeout_secs
    let interval = GrpcServerConfig::default_keepalive_interval_secs().unwrap();
    let timeout = GrpcServerConfig::default_keepalive_timeout_secs();
    assert!(
        timeout < interval,
        "timeout must be shorter than the keepalive interval"
    );
}

// ── new ─────────────────────────────────────────────────────────────────────

#[test]
fn test_new_requires_tls_by_default_happy() {
    // @covers: new
    let cfg = GrpcServerConfig::new(bind());
    assert!(cfg.tls_required, "new config must require TLS by default");
    assert_eq!(cfg.bind.port(), 50051);
}

#[test]
fn test_new_no_tls_attached_error() {
    // @covers: new
    let cfg = GrpcServerConfig::new(bind());
    assert!(cfg.tls.is_none(), "new config must have no TLS attached");
    assert!(
        !cfg.allow_unauthenticated,
        "new config must not allow unauthenticated"
    );
}

// ── allow_plaintext ─────────────────────────────────────────────────────────

#[test]
fn test_allow_plaintext_clears_tls_required_happy() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfig::new(bind()).allow_plaintext();
    assert!(!cfg.tls_required, "allow_plaintext must clear tls_required");
}

#[test]
fn test_allow_plaintext_new_default_is_tls_required_error() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfig::new(bind());
    assert!(
        cfg.tls_required,
        "without allow_plaintext, TLS must be required"
    );
}

#[test]
fn test_allow_plaintext_called_twice_stays_plaintext_edge() {
    // @covers: allow_plaintext
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .allow_plaintext();
    assert!(
        !cfg.tls_required,
        "double allow_plaintext must not re-enable TLS"
    );
}

// ── with_tls ────────────────────────────────────────────────────────────────

#[test]
fn test_with_tls_stores_config_happy() {
    // @covers: with_tls
    let tls = PemTlsConfig {
        cert_pem_path: "cert.pem".into(),
        key_pem_path: "key.pem".into(),
        ca_pem_path: None,
    };
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_tls(tls);
    assert!(cfg.tls.is_some(), "with_tls must store the TLS config");
    assert!(
        !cfg.tls_required,
        "allow_plaintext must remain in effect after with_tls"
    );
}

#[test]
fn test_with_tls_default_has_none_error() {
    // @covers: with_tls
    let cfg = GrpcServerConfig::new(bind());
    assert!(cfg.tls.is_none(), "default config must have no TLS");
}

#[test]
fn test_with_tls_overwrites_previous_config_edge() {
    // @covers: with_tls
    let tls1 = PemTlsConfig {
        cert_pem_path: "cert1.pem".into(),
        key_pem_path: "key1.pem".into(),
        ca_pem_path: None,
    };
    let tls2 = PemTlsConfig {
        cert_pem_path: "cert2.pem".into(),
        key_pem_path: "key2.pem".into(),
        ca_pem_path: None,
    };
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_tls(tls1)
        .with_tls(tls2);
    assert!(
        cfg.tls.is_some(),
        "second with_tls must overwrite the first"
    );
    assert!(
        !cfg.tls_required,
        "allow_plaintext must remain after double with_tls"
    );
}

// ── with_max_message_bytes ──────────────────────────────────────────────────

#[test]
fn test_with_max_message_bytes_overrides_default_happy() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_max_message_bytes(512);
    assert_eq!(cfg.max_message_bytes, 512);
}

#[test]
fn test_with_max_message_bytes_one_byte_edge() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_max_message_bytes(1);
    assert_eq!(cfg.max_message_bytes, 1, "must accept 1-byte cap");
}

#[test]
fn test_with_max_message_bytes_default_is_nonzero_error() {
    // @covers: with_max_message_bytes
    let cfg = GrpcServerConfig::new(bind());
    assert_ne!(
        cfg.max_message_bytes, 0,
        "default max_message_bytes must not be zero"
    );
}

// ── with_max_concurrent_streams ─────────────────────────────────────────────

#[test]
fn test_with_max_concurrent_streams_overrides_default_happy() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_max_concurrent_streams(8);
    assert_eq!(cfg.max_concurrent_streams, 8);
}

#[test]
fn test_with_max_concurrent_streams_default_is_nonzero_error() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfig::new(bind());
    assert_ne!(
        cfg.max_concurrent_streams, 0,
        "default max_concurrent_streams must not be zero"
    );
}

#[test]
fn test_with_max_concurrent_streams_value_of_one_edge() {
    // @covers: with_max_concurrent_streams
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_max_concurrent_streams(1);
    assert_eq!(cfg.max_concurrent_streams, 1, "must accept 1 as minimum");
    assert_ne!(cfg.max_concurrent_streams, 0);
}

// ── with_compression ────────────────────────────────────────────────────────

#[test]
fn test_with_compression_sets_gzip_happy() {
    // @covers: with_compression
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_compression(CompressionMode::Gzip);
    assert!(matches!(cfg.compression, CompressionMode::Gzip));
}

#[test]
fn test_with_compression_default_is_none_error() {
    // @covers: with_compression
    let cfg = GrpcServerConfig::new(bind());
    assert!(matches!(cfg.compression, CompressionMode::None));
}

#[test]
fn test_with_compression_overrides_gzip_back_to_none_edge() {
    // @covers: with_compression
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None);
    assert!(
        matches!(cfg.compression, CompressionMode::None),
        "compression must be overridable back to None"
    );
}

// ── with_keepalive ──────────────────────────────────────────────────────────

#[test]
fn test_with_keepalive_sets_interval_and_timeout_happy() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_keepalive(30, 5);
    assert_eq!(cfg.keepalive_interval_secs, Some(30));
    assert_eq!(cfg.keepalive_timeout_secs, 5);
}

#[test]
fn test_with_keepalive_zero_interval_disables_it_edge() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_keepalive(0, 5);
    assert_eq!(
        cfg.keepalive_interval_secs, None,
        "interval_secs=0 must disable keepalive"
    );
}

#[test]
fn test_with_keepalive_large_values_accepted_error() {
    // @covers: with_keepalive
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .with_keepalive(3600, 60);
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
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .without_keepalive();
    assert!(
        cfg.keepalive_interval_secs.is_none(),
        "without_keepalive must clear the interval"
    );
    assert_ne!(
        cfg.keepalive_interval_secs,
        Some(0),
        "must be None not zero"
    );
}

#[test]
fn test_without_keepalive_default_has_interval_error() {
    // @covers: without_keepalive
    let cfg = GrpcServerConfig::new(bind());
    assert!(
        cfg.keepalive_interval_secs.is_some(),
        "default must have keepalive enabled"
    );
    assert_ne!(
        cfg.keepalive_interval_secs,
        Some(0),
        "default keepalive must not be zero secs"
    );
}

#[test]
fn test_without_keepalive_called_twice_stays_disabled_edge() {
    // @covers: without_keepalive
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .without_keepalive()
        .without_keepalive();
    assert!(
        cfg.keepalive_interval_secs.is_none(),
        "double without_keepalive must remain disabled"
    );
}

// ── allow_unauthenticated ───────────────────────────────────────────────────

#[test]
fn test_allow_unauthenticated_sets_flag_happy() {
    // @covers: allow_unauthenticated
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .allow_unauthenticated();
    assert!(
        cfg.allow_unauthenticated,
        "allow_unauthenticated must set the flag"
    );
}

#[test]
fn test_allow_unauthenticated_default_is_false_error() {
    // @covers: allow_unauthenticated
    let cfg = GrpcServerConfig::new(bind());
    assert!(
        !cfg.allow_unauthenticated,
        "default must NOT allow unauthenticated"
    );
}

#[test]
fn test_allow_unauthenticated_with_tls_still_sets_flag_edge() {
    // @covers: allow_unauthenticated
    let tls = PemTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        ca_pem_path: None,
    };
    let cfg = GrpcServerConfig::new(bind())
        .with_tls(tls)
        .allow_unauthenticated();
    assert!(
        cfg.allow_unauthenticated,
        "allow_unauthenticated must work alongside TLS"
    );
}

// ── enable_reflection ───────────────────────────────────────────────────────

#[test]
fn test_enable_reflection_sets_flag_happy() {
    // @covers: enable_reflection
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .enable_reflection();
    assert!(cfg.enable_reflection, "enable_reflection must set the flag");
}

#[test]
fn test_enable_reflection_default_is_false_error() {
    // @covers: enable_reflection
    let cfg = GrpcServerConfig::new(bind());
    assert!(!cfg.enable_reflection, "reflection must be off by default");
}

#[test]
fn test_enable_reflection_with_keepalive_coexist_edge() {
    // @covers: enable_reflection
    let cfg = GrpcServerConfig::new(bind())
        .allow_plaintext()
        .enable_reflection()
        .with_keepalive(20, 4);
    assert!(
        cfg.enable_reflection,
        "reflection flag must survive chaining with keepalive"
    );
    assert_eq!(cfg.keepalive_interval_secs, Some(20));
}
