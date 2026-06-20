//! No-op [`Validator`] implementation for gRPC ingress.

use crate::api::{GrpcIngressError, NoopGrpcValidator, Validator};

impl Validator for NoopGrpcValidator {
    fn validate(&self) -> Result<(), GrpcIngressError> {
        Ok(())
    }
}
