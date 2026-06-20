//! [`Validator`] — validates gRPC ingress configuration or requests.

use crate::api::GrpcIngressError;

/// Validates gRPC ingress configuration or request payloads.
///
/// Implementors check for well-formedness before a request is dispatched
/// to the underlying [`super::GrpcIngress`] handler.
pub trait Validator {
    /// Validate the target. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), GrpcIngressError>;
}
