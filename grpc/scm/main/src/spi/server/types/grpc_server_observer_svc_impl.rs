//! GrpcServerObserverOps trait impl for GrpcServerObserverSvc.
use crate::api::{GrpcServerObserver, GrpcServerObserverOps, GrpcServerObserverSvc};

impl GrpcServerObserverOps for GrpcServerObserverSvc {
    fn is_reflection_enabled(server: &dyn GrpcServerObserver) -> bool {
        server.is_reflection_enabled()
    }
}
