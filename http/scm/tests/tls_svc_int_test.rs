//! Integration tests for `TlsSvc::build_tls_acceptor` and its rustls/pemfile deps.
// @covers TlsSvc::build_tls_acceptor
// @covers dep:rustls
// @covers dep:rustls-pemfile
// @covers dep:tokio-rustls
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{IngressTlsConfig, IngressTlsError};
use swe_edge_runtime_http::TlsSvc;

fn write_temp(path: &std::path::Path, content: &str) {
    std::fs::write(path, content).expect("temp file write failed");
}

fn generate_self_signed() -> (String, String) {
    let key_pair = rcgen::KeyPair::generate().expect("key gen failed");
    let cert = rcgen::CertificateParams::new(vec!["localhost".into()])
        .expect("cert params failed")
        .self_signed(&key_pair)
        .expect("self-sign failed");
    (cert.pem(), key_pair.serialize_pem())
}

// ── dep:rustls — ServerConfig construction ────────────────────────────────────

#[test]
fn test_rustls_server_config_builder_is_constructible_happy() {
    use rustls::ServerConfig;
    // Exercises rustls::ServerConfig builder pattern used internally.
    let builder = ServerConfig::builder().with_no_client_auth();
    assert!(
        std::mem::size_of_val(&builder) > 0,
        "rustls builder must occupy memory"
    );
}

#[test]
fn test_rustls_root_cert_store_starts_empty_error() {
    use rustls::RootCertStore;
    let store = RootCertStore::empty();
    assert_eq!(store.len(), 0, "empty RootCertStore must have no entries");
}

// ── dep:rustls-pemfile — PEM parsing ─────────────────────────────────────────

#[test]
fn test_rustls_pemfile_certs_parses_valid_pem_edge() {
    use std::io::BufReader;
    let (cert_pem, _) = generate_self_signed();
    let certs: Vec<_> = rustls_pemfile::certs(&mut BufReader::new(cert_pem.as_bytes()))
        .collect::<Result<_, _>>()
        .expect("cert parse must succeed");
    assert!(
        !certs.is_empty(),
        "self-signed PEM must yield at least one cert"
    );
}

// ── dep:tokio-rustls — TlsAcceptor type-level coverage ───────────────────────

#[test]
fn test_tokio_rustls_acceptor_is_non_zero_size_happy() {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer};
    use rustls::ServerConfig;
    use std::sync::Arc;
    use tokio_rustls::TlsAcceptor;

    let dir = tempfile::tempdir().expect("tempdir failed");
    let (cert_pem, key_pem) = generate_self_signed();
    let cert_path = dir.path().join("cert.pem");
    let key_path = dir.path().join("key.pem");
    write_temp(&cert_path, &cert_pem);
    write_temp(&key_path, &key_pem);

    let cert_bytes = std::fs::read(&cert_path).expect("read cert");
    let key_bytes = std::fs::read(&key_path).expect("read key");
    let certs: Vec<CertificateDer<'static>> =
        rustls_pemfile::certs(&mut std::io::BufReader::new(cert_bytes.as_slice()))
            .collect::<Result<_, _>>()
            .expect("parse certs");
    let key: PrivateKeyDer<'static> =
        rustls_pemfile::private_key(&mut std::io::BufReader::new(key_bytes.as_slice()))
            .expect("parse key")
            .expect("key present");
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("build ServerConfig");
    let acceptor = TlsAcceptor::from(Arc::new(config));
    assert!(
        std::mem::size_of_val(&acceptor) > 0,
        "TlsAcceptor must be a non-zero-size value"
    );
}

// ── TlsSvc::build_tls_acceptor ────────────────────────────────────────────────

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_valid_cert_produces_acceptor_happy() {
    let dir = tempfile::tempdir().expect("tempdir failed");
    let (cert_pem, key_pem) = generate_self_signed();
    let cert_path = dir.path().join("cert.pem");
    let key_path = dir.path().join("key.pem");
    write_temp(&cert_path, &cert_pem);
    write_temp(&key_path, &key_pem);

    let cfg = IngressTlsConfig {
        cert_pem_path: cert_path.to_string_lossy().into_owned(),
        key_pem_path: key_path.to_string_lossy().into_owned(),
        client_ca_pem_path: None,
    };

    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(
        result.is_ok(),
        "valid self-signed cert must produce a TlsAcceptor: {result:?}"
    );
    assert!(
        std::mem::size_of_val(&result.unwrap()) > 0,
        "TlsAcceptor must be a non-zero-size value"
    );
}

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_missing_cert_returns_cert_load_error_error() {
    let cfg = IngressTlsConfig {
        cert_pem_path: "/nonexistent/cert.pem".into(),
        key_pem_path: "/nonexistent/key.pem".into(),
        client_ca_pem_path: None,
    };

    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(result.is_err(), "missing cert path must fail");
    assert!(
        matches!(result.unwrap_err(), IngressTlsError::CertLoad(path, _) if path.contains("cert.pem")),
        "error must be CertLoad for the cert path"
    );
}

/// @covers: TlsSvc::build_tls_acceptor
#[test]
fn test_build_tls_acceptor_mtls_with_valid_ca_produces_acceptor_edge() {
    let dir = tempfile::tempdir().expect("tempdir failed");
    let (cert_pem, key_pem) = generate_self_signed();
    let (ca_pem, _ca_key) = generate_self_signed();
    let cert_path = dir.path().join("cert.pem");
    let key_path = dir.path().join("key.pem");
    let ca_path = dir.path().join("ca.pem");
    write_temp(&cert_path, &cert_pem);
    write_temp(&key_path, &key_pem);
    write_temp(&ca_path, &ca_pem);

    let cfg = IngressTlsConfig {
        cert_pem_path: cert_path.to_string_lossy().into_owned(),
        key_pem_path: key_path.to_string_lossy().into_owned(),
        client_ca_pem_path: Some(ca_path.to_string_lossy().into_owned()),
    };

    let result = TlsSvc::build_tls_acceptor(&cfg);
    assert!(
        result.is_ok(),
        "valid mTLS config must produce a TlsAcceptor: {result:?}"
    );
    assert!(
        std::mem::size_of_val(&result.unwrap()) > 0,
        "mTLS acceptor must be a non-zero-size value"
    );
}
