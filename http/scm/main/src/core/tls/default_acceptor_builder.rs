//! TLS acceptor construction using rustls + tokio-rustls.

use std::fs;
use std::io::BufReader;
use std::sync::Arc;

use edge_domain_security::{IngressTlsError, PemTlsConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

pub(crate) struct DefaultAcceptorBuilder;

impl DefaultAcceptorBuilder {
    pub(crate) fn build_tls_acceptor(cfg: &PemTlsConfig) -> Result<TlsAcceptor, IngressTlsError> {
        let certs = Self::load_certs(&cfg.cert_pem_path)?;
        let key = Self::load_key(&cfg.key_pem_path)?;

        let config = if let Some(ca_path) = &cfg.ca_pem_path {
            Self::build_mtls_config(certs, key, ca_path)?
        } else {
            ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(certs, key)
                .map_err(|e| IngressTlsError::Config(e.to_string()))?
        };

        Ok(TlsAcceptor::from(Arc::new(config)))
    }

    fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, IngressTlsError> {
        let bytes = fs::read(path).map_err(|e| IngressTlsError::CertLoad(path.to_owned(), e))?;
        let certs: Vec<_> = rustls_pemfile::certs(&mut BufReader::new(bytes.as_slice()))
            .collect::<Result<_, _>>()
            .map_err(|_| IngressTlsError::CertParse(path.to_owned()))?;
        if certs.is_empty() {
            return Err(IngressTlsError::CertParse(path.to_owned()));
        }
        Ok(certs)
    }

    fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, IngressTlsError> {
        let bytes = fs::read(path).map_err(|e| IngressTlsError::KeyLoad(path.to_owned(), e))?;
        rustls_pemfile::private_key(&mut BufReader::new(bytes.as_slice()))
            .map_err(|_| IngressTlsError::KeyParse(path.to_owned()))?
            .ok_or_else(|| IngressTlsError::KeyParse(path.to_owned()))
    }

    fn build_mtls_config(
        certs: Vec<CertificateDer<'static>>,
        key: PrivateKeyDer<'static>,
        ca_path: &str,
    ) -> Result<ServerConfig, IngressTlsError> {
        let ca_certs = Self::load_certs(ca_path)?;
        let mut root_store = rustls::RootCertStore::empty();
        for ca in ca_certs {
            root_store
                .add(ca)
                .map_err(|e| IngressTlsError::Config(e.to_string()))?;
        }
        let client_auth = rustls::server::WebPkiClientVerifier::builder(Arc::new(root_store))
            .build()
            .map_err(|e| IngressTlsError::Config(e.to_string()))?;
        ServerConfig::builder()
            .with_client_cert_verifier(client_auth)
            .with_single_cert(certs, key)
            .map_err(|e| IngressTlsError::Config(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultAcceptorBuilder;
    use edge_domain_security::{IngressTlsError, PemTlsConfig};

    fn generate_self_signed() -> (String, String) {
        let key_pair = rcgen::KeyPair::generate().expect("key gen");
        let cert = rcgen::CertificateParams::new(vec!["localhost".into()])
            .expect("params")
            .self_signed(&key_pair)
            .expect("sign");
        (cert.pem(), key_pair.serialize_pem())
    }

    fn write_temp(dir: &std::path::Path, name: &str, content: &str) -> String {
        let path = dir.join(name);
        std::fs::write(&path, content).expect("write");
        path.to_string_lossy().into_owned()
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_valid_cert_and_key_returns_acceptor_happy() {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
        let dir = tempfile::tempdir().expect("tempdir");
        let (cert, key) = generate_self_signed();
        let cfg = PemTlsConfig {
            cert_pem_path: write_temp(dir.path(), "c.pem", &cert),
            key_pem_path: write_temp(dir.path(), "k.pem", &key),
            ca_pem_path: None,
        };
        let result = DefaultAcceptorBuilder::build_tls_acceptor(&cfg);
        assert!(result.is_ok(), "valid cert+key must produce acceptor");
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_missing_cert_returns_cert_load_error_error() {
        let cfg = PemTlsConfig {
            cert_pem_path: "/no/cert.pem".into(),
            key_pem_path: "/no/key.pem".into(),
            ca_pem_path: None,
        };
        let result = DefaultAcceptorBuilder::build_tls_acceptor(&cfg);
        assert!(
            matches!(result, Err(IngressTlsError::CertLoad(_, _))),
            "missing cert must be CertLoad error"
        );
    }

    /// @covers: build_tls_acceptor
    #[test]
    fn test_build_tls_acceptor_mtls_with_ca_produces_acceptor_edge() {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
        let dir = tempfile::tempdir().expect("tempdir");
        let (cert, key) = generate_self_signed();
        let (ca, _) = generate_self_signed();
        let cfg = PemTlsConfig {
            cert_pem_path: write_temp(dir.path(), "c.pem", &cert),
            key_pem_path: write_temp(dir.path(), "k.pem", &key),
            ca_pem_path: Some(write_temp(dir.path(), "ca.pem", &ca)),
        };
        let result = DefaultAcceptorBuilder::build_tls_acceptor(&cfg);
        assert!(result.is_ok(), "valid mTLS config must produce acceptor");
    }
}
