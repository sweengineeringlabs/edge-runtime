//! Result type alias for gRPC ingress operations.

use crate::api::GrpcIngressError;

/// Result type for [`crate::GrpcIngress`] operations.
pub type GrpcIngressResult<T> = Result<T, GrpcIngressError>;
