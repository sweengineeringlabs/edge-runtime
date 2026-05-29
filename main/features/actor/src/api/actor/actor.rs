//! Actor trait — message-handling state machine.

use futures::future::BoxFuture;

use crate::api::types::actor::actor_context::ActorContext;

/// An actor is an encapsulated state machine that processes messages sequentially.
///
/// Actors own their internal state and process each message to completion before
/// handling the next. Messages are processed via the `handle` method, which receives
/// an `ActorContext` for tell (fire-and-forget) and ask (request-reply) operations.
///
/// # Example
/// ```ignore
/// struct Counter {
///     count: i32,
/// }
///
/// impl Actor for Counter {
///     type Message = CounterMessage;
///
///     async fn handle(&mut self, ctx: ActorContext<Self>, msg: Self::Message) {
///         match msg {
///             CounterMessage::Increment => self.count += 1,
///             CounterMessage::GetCount(reply) => {
///                 let _ = reply.send(self.count);
///             }
///         }
///     }
/// }
/// ```
pub trait Actor: Send + 'static {
    /// The message type this actor processes.
    type Message: Send + 'static;

    /// Handle a single message and update internal state.
    ///
    /// This method receives:
    /// - `self`: mutable access to internal state
    /// - `ctx`: context for sending messages to other actors
    /// - `msg`: the message to process
    ///
    /// Processing is sequential — the next message waits until this completes.
    fn handle(&mut self, ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()>
    where
        Self: Sized;

    /// Called when the actor stops (optional lifecycle hook).
    ///
    /// Override to clean up resources (close files, flush state, etc.).
    /// Default implementation does nothing.
    fn on_stop(&mut self) -> BoxFuture<'_, ()>
    where
        Self: Sized,
    {
        Box::pin(async {})
    }
}
