//! Actor handle for sending messages.

use futures::future::BoxFuture;

use crate::api::error::mailbox_error::MailboxError;

/// A handle to send messages to an actor.
///
/// `ActorHandle` allows fire-and-forget message sends via `tell`. Cloning an
/// `ActorHandle` creates additional sender endpoints to the same actor.
/// The concrete impl is `ActorSpawnHandle<A>`, returned by `ActorRuntime::spawn`.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_runtime_actor::{ActorHandle, MailboxError};
///
/// async fn send_once<H, M>(handle: &H, msg: M) -> Result<(), MailboxError>
/// where
///     H: ActorHandle<M>,
///     M: Send + 'static,
/// {
///     handle.tell(msg).await
/// }
/// ```
pub trait ActorHandle<M: Send + 'static>: Clone + Send {
    /// Send a message without waiting for a response (fire-and-forget).
    ///
    /// Returns `Ok(())` if the message was enqueued, or `MailboxError` if the
    /// mailbox is full, closed, or the actor stopped.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // Obtain a handle from ActorRuntime::spawn, then:
    /// // handle.tell(MyMessage::Ping).await?;
    /// ```
    fn tell(&self, msg: M) -> BoxFuture<'_, Result<(), MailboxError>>;
}
