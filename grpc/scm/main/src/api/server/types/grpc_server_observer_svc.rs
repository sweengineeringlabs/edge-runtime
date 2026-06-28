//! GrpcServerObserverSvc type declaration and inherent impl.

use crate::api::GrpcServerObserver;

/// Observer factory for gRPC servers.
pub struct GrpcServerObserverSvc;

impl GrpcServerObserverSvc {
    /// Returns whether reflection is enabled by observing a GrpcServer.
    pub fn is_reflection_enabled(server: &dyn GrpcServerObserver) -> bool {
        server.is_reflection_enabled()
    }
}
