//! gRPC method streaming type classification.

/// Classifies the streaming pattern of a gRPC RPC method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpcMethod {
    /// Single request, single response.
    Unary,
    /// Single request, stream of responses from server.
    ServerStream,
    /// Stream of requests from client, single response.
    ClientStream,
    /// Bidirectional streaming.
    BidiStream,
}
