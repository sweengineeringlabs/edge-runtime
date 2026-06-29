//! External tests for [`crate::TlsSvc`] SAF facade.

#[cfg(test)]
mod tests {
    use crate::api::TlsSvc;
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

    /// @covers: TlsSvc::build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_valid_pem_returns_acceptor_happy() {
        let dir = tempfile::tempdir().expect("tempdir");
        let (cert, key) = write_pems(&dir);
        let cfg = PemTlsConfig::tls(cert, key);
        assert_eq!(
            TlsSvc::build_tls_acceptor(&cfg).is_ok(),
            true,
            "valid PEM paths must produce a TlsAcceptor"
        );
    }

    /// @covers: TlsSvc::build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_missing_cert_returns_error_error() {
        let cfg = PemTlsConfig::tls("/no/cert.pem", "/no/key.pem");
        assert!(
            matches!(
                TlsSvc::build_tls_acceptor(&cfg),
                Err(IngressTlsError::CertLoad(_, _))
            ),
            "missing cert path must return CertLoad error"
        );
    }

    /// @covers: TlsSvc::build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_mtls_config_returns_acceptor_edge() {
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
        assert_eq!(
            TlsSvc::build_tls_acceptor(&cfg).is_ok(),
            true,
            "mTLS config must produce an acceptor"
        );
    }
}
