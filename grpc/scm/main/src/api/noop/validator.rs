//! Validation contract for gRPC server configurations and interceptors.

use crate::api::noop::noop_grpc_validator::NoopGrpcValidator;
use crate::api::noop::validator_svc::ValidatorSvc;

use crate::api::GrpcValidationError;

/// Validation contract for gRPC server configurations and interceptors.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a structured validation error.
    fn validate(&self) -> Result<(), GrpcValidationError>;

    /// Return a noop validator instance (type anchor for [`NoopGrpcValidator`]).
    fn new_noop() -> NoopGrpcValidator
    where
        Self: Sized,
    {
        NoopGrpcValidator
    }

    /// Return a factory for validators (type anchor for [`ValidatorSvc`]).
    fn new_svc() -> ValidatorSvc
    where
        Self: Sized,
    {
        ValidatorSvc
    }
}
