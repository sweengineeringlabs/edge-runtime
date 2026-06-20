//! Outbound gRPC response value type.

use std::collections::HashMap;

/// A gRPC response produced by a [`crate::GrpcIngress`] handler.
#[derive(Debug, Clone)]
pub struct GrpcResponse {
    /// gRPC status code (0 = OK).
    pub status: u32,
    /// Raw serialized response body.
    pub body: Vec<u8>,
    /// Response metadata (analogous to HTTP trailers/headers).
    pub metadata: HashMap<String, String>,
}

impl GrpcResponse {
    /// Construct a successful response with the given body.
    pub fn ok(body: Vec<u8>) -> Self {
        Self {
            status: 0,
            body,
            metadata: HashMap::new(),
        }
    }

    /// Construct an empty successful response.
    pub fn empty() -> Self {
        Self::ok(vec![])
    }

    /// Returns `true` when `status == 0` (gRPC OK).
    pub fn is_ok(&self) -> bool {
        self.status == 0
    }
}
