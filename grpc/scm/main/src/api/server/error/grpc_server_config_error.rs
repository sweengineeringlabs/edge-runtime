//! Configuration error for [`GrpcServerConfig`](crate::api::server::types::GrpcServerConfig).

/// Error returned when [`GrpcServerConfig`](crate::api::server::types::GrpcServerConfig) is invalid.
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerConfigError {
    /// `tls_required` is set but no TLS configuration was supplied.
    #[error(
        "tls_required is set but no TLS configuration supplied — \
         call `.with_tls(cfg)` before `.serve()`"
    )]
    TlsRequiredButMissing,
}
