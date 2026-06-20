//! Zero-size marker type for the no-op gRPC ingress implementation.

/// A pass-through gRPC ingress handler that always returns an empty OK response.
///
/// Useful for tests and composition roots that have not yet wired a real ingress
/// implementation.
pub struct NoopGrpcIngress;
