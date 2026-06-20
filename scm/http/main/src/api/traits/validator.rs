//! [`Validator`] — validates HTTP ingress configuration or requests.

use crate::api::HttpIngressError;

/// Validates HTTP ingress configuration or request payloads.
///
/// Implementors check for well-formedness before a request is dispatched
/// to the underlying [`super::HttpIngress`] handler.
pub trait Validator {
    /// Validate the target. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), HttpIngressError>;
}
