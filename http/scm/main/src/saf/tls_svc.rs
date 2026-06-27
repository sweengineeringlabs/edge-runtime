//! SAF surface for [`TlsSvc`] — TLS acceptor factory.

use edge_domain_security::{IngressTlsConfig, IngressTlsError};
use tokio_rustls::TlsAcceptor;

use crate::api::TlsSvc;
use crate::core::tls::DefaultAcceptorBuilder;

/// Service identifier for the TLS acceptor factory.
pub const TLS_SVC: &str = "tls";

impl TlsSvc {
    /// Build a [`TlsAcceptor`] from an [`IngressTlsConfig`].
    pub fn build_tls_acceptor(cfg: &IngressTlsConfig) -> Result<TlsAcceptor, IngressTlsError> {
        DefaultAcceptorBuilder::build_tls_acceptor(cfg)
    }
}
