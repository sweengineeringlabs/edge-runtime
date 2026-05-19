//! Actor handle for sending messages.

use futures::future::BoxFuture;

use super::mailbox_error::MailboxError;

/// A handle to send messages to an actor.
///
/// `ActorHandle` allows fire-and-forget message sends via `tell`. Cloning an
/// `ActorHandle` creates additional sender endpoints to the same actor.
pub trait ActorHandle<M: Send + 'static>: Clone + Send {
    /// Send a message without waiting for a response (fire-and-forget).
    ///
    /// Returns `Ok(())` if the message was enqueued, or `MailboxError` if the
    /// mailbox is full, closed, or the actor stopped.
    fn tell(&self, msg: M) -> BoxFuture<'_, Result<(), MailboxError>>;
}
