//! Error type for gRPC validation failures.

/// Validation errors produced by [`crate::api::Validator`] implementations.
#[derive(Debug, PartialEq)]
pub enum GrpcValidationError {
    /// The value failed validation with the given human-readable message.
    Invalid(String),
}
