//! SAF surface for [`TlsSvc`] — TLS acceptor factory.

use edge_domain_security::{IngressTlsError, PemTlsConfig};
use tokio_rustls::TlsAcceptor;

use crate::api::TlsSvc;
use crate::core::tls::DefaultAcceptorBuilder;

/// Service identifier for the TLS acceptor factory.
pub const TLS_SVC: &str = "tls";

impl TlsSvc {
    /// Build a [`TlsAcceptor`] from an [`PemTlsConfig`].
    pub fn build_tls_acceptor(cfg: &PemTlsConfig) -> Result<TlsAcceptor, IngressTlsError> {
        DefaultAcceptorBuilder::build_tls_acceptor(cfg)
    }
}
