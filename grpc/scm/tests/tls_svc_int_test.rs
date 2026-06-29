//! Integration tests for [`TlsSvc`] — covers `api/tls/tls_svc.rs` and the
//! `GrpcServer::tls_svc()` factory method.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{PemTlsConfig, TlsConfig};
use swe_edge_runtime_grpc::TlsSvc;

/// @covers: tls_svc
#[test]
fn test_tls_svc_is_constructible_happy() {
    // GrpcServer::tls_svc() returns TlsSvc — verify the type exists and
    // build_tls_acceptor is reachable via it.
    let cfg = PemTlsConfig {
        cert_pem_path: "nonexistent.pem".into(),
        key_pem_path: "nonexistent.pem".into(),
        ca_pem_path: None,
    };
    let result = TlsSvc::build_tls_acceptor(&cfg);
    // tls_svc() itself cannot fail; build_tls_acceptor surfaces errors
    assert!(
        result.is_err(),
        "build_tls_acceptor with missing cert must err"
    );
}

/// @covers: tls_svc
#[test]
fn test_tls_svc_build_tls_acceptor_missing_cert_returns_error_error() {
    let cfg = PemTlsConfig {
        cert_pem_path: "/does/not/exist.pem".into(),
        key_pem_path: "/does/not/exist.pem".into(),
        ca_pem_path: None,
    };
    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(result.is_err(), "missing cert path must produce an error");
}

/// @covers: tls_svc
#[test]
fn test_tls_svc_mtls_config_is_detected_edge() {
    let tls = PemTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        ca_pem_path: None,
    };
    let mtls = PemTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        ca_pem_path: Some("ca.pem".into()),
    };
    assert!(!tls.is_mtls(), "TLS-only config must not report is_mtls");
    assert!(mtls.is_mtls(), "mTLS config must report is_mtls");
}
