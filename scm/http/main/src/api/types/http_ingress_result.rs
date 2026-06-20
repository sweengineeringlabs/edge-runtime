//! Result type alias for HTTP ingress operations.

use crate::api::error::HttpIngressError;

/// Result type for HTTP ingress operations.
pub type HttpIngressResult<T> = Result<T, HttpIngressError>;
