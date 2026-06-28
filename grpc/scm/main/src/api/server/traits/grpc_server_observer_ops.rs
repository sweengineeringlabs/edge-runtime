//! Operations that can be performed via a gRPC server observer.

use crate::api::GrpcServerObserver;

/// Operations that can be performed via [`GrpcServerObserverSvc`] as a service object.
///
/// Consuming / static methods are gated on `where Self: Sized` to keep the
/// trait object-safe via [`GrpcServerObserverOps::svc_marker`].
pub trait GrpcServerObserverOps {
    /// Return whether reflection is enabled on the given server.
    fn is_reflection_enabled(server: &dyn GrpcServerObserver) -> bool
    where
        Self: Sized;
    /// Object-safe marker method (required for trait-object coercions).
    fn svc_marker(&self) -> bool {
        true
    }
}
