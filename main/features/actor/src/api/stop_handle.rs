//! Actor lifecycle management.

use futures::future::BoxFuture;

/// A handle to stop an actor gracefully.
///
/// `StopHandle` provides a way to signal an actor to stop processing and shut down.
/// This is used when an actor needs lifecycle management beyond automatic cleanup.
pub trait StopHandle: Clone + Send {
    /// Signal the actor to stop and wait for shutdown.
    ///
    /// The actor will finish processing the current message, call `on_stop()`,
    /// and then the task will complete.
    fn stop(&self) -> BoxFuture<'_, ()>;
}
