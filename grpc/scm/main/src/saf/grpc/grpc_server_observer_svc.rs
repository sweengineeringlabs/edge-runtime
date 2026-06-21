//! SAF factory surface for GrpcServerObserver.

use crate::api::{GrpcServerObserver, GrpcServerObserverSvc};

impl GrpcServerObserverSvc {
    /// Returns whether reflection is enabled by observing a GrpcServer.
    pub fn is_reflection_enabled(server: &dyn GrpcServerObserver) -> bool {
        server.is_reflection_enabled()
    }
}
