//! Integration coverage for the `edge-security-runtime` dependency.
//!
//! The TLS acceptor builder lives in edge-security-runtime; http re-exports
//! `TlsSvc` and calls `build_tls_acceptor` from its TLS serve path. This test
//! imports the crate directly and exercises the factory's error path (which
//! needs no crypto provider). The happy/mTLS paths are covered in
//! edge-security-runtime's own suite where the rustls provider is installed.
// @covers: edge-security-runtime / TlsSvc::build_tls_acceptor
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{IngressTlsError, PemTlsConfig};
use edge_security_runtime::TlsSvc;

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_missing_cert_returns_cert_load_error_error() {
    let cfg = PemTlsConfig {
        cert_pem_path: "/nonexistent/cert.pem".into(),
        key_pem_path: "/nonexistent/key.pem".into(),
        ca_pem_path: None,
    };

    let result = TlsSvc::build_tls_acceptor(&cfg);
    let err = result.err().expect("missing cert path must fail");
    assert!(
        matches!(err, IngressTlsError::CertLoad(ref path, _) if path.contains("cert.pem")),
        "missing cert must surface as CertLoad for the cert path"
    );
}

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_present_but_invalid_cert_returns_cert_parse_edge() {
    // A cert file that exists but holds non-PEM content gets past the read and
    // fails at parse — distinguishing CertParse (bad content) from CertLoad
    // (missing file) and proving the builder validates content, not just paths.
    let dir = tempfile::tempdir().expect("tempdir");
    let cert_path = dir.path().join("cert.pem");
    std::fs::write(&cert_path, "not a real certificate").expect("write cert");

    let cfg = PemTlsConfig {
        cert_pem_path: cert_path.to_string_lossy().into_owned(),
        key_pem_path: "/nonexistent/key.pem".into(),
        ca_pem_path: None,
    };

    let err = TlsSvc::build_tls_acceptor(&cfg)
        .err()
        .expect("garbage cert content must fail");
    assert!(
        matches!(err, IngressTlsError::CertParse(ref p) if p.contains("cert.pem")),
        "present-but-garbage cert must surface as CertParse, got: {err:?}"
    );
}
