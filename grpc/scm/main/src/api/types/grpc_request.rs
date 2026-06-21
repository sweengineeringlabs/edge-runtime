//! Inbound gRPC request value type.

use std::collections::HashMap;

/// An inbound gRPC request received by a [`crate::GrpcIngress`] handler.
#[derive(Debug, Clone)]
pub struct GrpcRequest {
    /// The fully-qualified service name (e.g. `acme.greeter.GreeterService`).
    pub service: String,
    /// The RPC method name (e.g. `SayHello`).
    pub method: String,
    /// Raw serialized request body (Protobuf bytes or JSON, handler decides).
    pub body: Vec<u8>,
    /// Request metadata (analogous to HTTP headers).
    pub metadata: HashMap<String, String>,
}

impl GrpcRequest {
    /// Construct a minimal request with service, method, and body.
    pub fn new(service: impl Into<String>, method: impl Into<String>, body: Vec<u8>) -> Self {
        Self {
            service: service.into(),
            method: method.into(),
            body,
            metadata: HashMap::new(),
        }
    }

    /// Attach a metadata entry.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
