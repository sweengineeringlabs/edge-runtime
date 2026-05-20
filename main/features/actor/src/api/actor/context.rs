//! Actor execution context.

use std::marker::PhantomData;

use super::mailbox::Actor;

/// Execution context passed to an actor's `handle` method.
///
/// `ActorContext` provides access to the actor's address and environment during
/// message processing. Currently a placeholder for future extensibility (metrics,
/// tracing context, parent actor references, etc.).
pub struct ActorContext<A: Actor> {
    _marker: PhantomData<A>,
}

impl<A: Actor> Clone for ActorContext<A> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A: Actor> ActorContext<A> {
    /// Create a new actor context.
    pub(crate) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
