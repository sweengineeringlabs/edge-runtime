//! Colocated tests for [`DefaultAcceptorBuilder::build_tls_acceptor`].

#[cfg(test)]
mod tests {
    use crate::core::tls::DefaultAcceptorBuilder;
    use edge_domain_security::{IngressTlsError, PemTlsConfig};
    use std::fs;
    use tempfile::TempDir;

    fn write_pems(dir: &TempDir) -> (String, String) {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
        let key_pair = rcgen::KeyPair::generate().expect("key gen");
        let cert = rcgen::CertificateParams::new(vec!["localhost".into()])
            .expect("params")
            .self_signed(&key_pair)
            .expect("sign");
        let cert_path = dir.path().join("cert.pem");
        let key_path = dir.path().join("key.pem");
        fs::write(&cert_path, cert.pem()).expect("write cert");
        fs::write(&key_path, key_pair.serialize_pem()).expect("write key");
        (
            cert_path.to_string_lossy().into_owned(),
            key_path.to_string_lossy().into_owned(),
        )
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_valid_pem_pair_returns_acceptor_happy() {
        let dir = tempfile::tempdir().expect("tempdir");
        let (cert, key) = write_pems(&dir);
        let cfg = PemTlsConfig::tls(cert, key);
        assert!(
            DefaultAcceptorBuilder::build_tls_acceptor(&cfg).is_ok(),
            "valid TLS config must produce an acceptor"
        );
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_missing_cert_returns_cert_load_error_error() {
        let cfg = PemTlsConfig::tls("/nonexistent/cert.pem", "/nonexistent/key.pem");
        let result = DefaultAcceptorBuilder::build_tls_acceptor(&cfg);
        assert!(
            matches!(result, Err(IngressTlsError::CertLoad(_, _))),
            "missing cert must return CertLoad error"
        );
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_with_ca_path_returns_acceptor_edge() {
        let dir = tempfile::tempdir().expect("tempdir");
        let (cert, key) = write_pems(&dir);
        let ca_path = dir.path().join("ca.pem");
        fs::write(&ca_path, {
            let key_pair = rcgen::KeyPair::generate().expect("key gen");
            let cert = rcgen::CertificateParams::new(vec!["ca".into()])
                .expect("ca params")
                .self_signed(&key_pair)
                .expect("ca sign");
            cert.pem()
        })
        .expect("write ca");
        let cfg = PemTlsConfig::mtls(cert, key, ca_path.to_string_lossy().into_owned());
        assert!(
            DefaultAcceptorBuilder::build_tls_acceptor(&cfg).is_ok(),
            "mTLS config with valid CA must produce an acceptor"
        );
    }
}
