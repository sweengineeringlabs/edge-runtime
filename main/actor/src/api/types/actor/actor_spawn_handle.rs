//! Concrete actor handle returned by [`crate::api::ActorRuntime::spawn`].

#[cfg(feature = "tokio-rt")]
use std::sync::Arc;

#[cfg(feature = "tokio-rt")]
use futures::future::BoxFuture;

#[cfg(feature = "tokio-rt")]
use tokio::sync::mpsc;

#[cfg(feature = "tokio-rt")]
use crate::api::{Actor, MailboxError};

#[cfg(feature = "tokio-rt")]
use crate::api::actor::actor_handle::ActorHandle;

#[cfg(feature = "tokio-rt")]
use crate::api::types::message::Message;

/// Concrete handle to an actor spawned with [`crate::api::ActorRuntime::spawn`].
///
/// Holds the tokio channel sender directly so `api/types/` does not depend on
/// `spi/`.  Clone is cheap — it shares the underlying channel via `Arc`.
#[cfg(feature = "tokio-rt")]
pub struct ActorSpawnHandle<A: Actor> {
    pub(crate) tx: Arc<mpsc::Sender<Message<A>>>,
}

#[cfg(feature = "tokio-rt")]
impl<A: Actor> Clone for ActorSpawnHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

#[cfg(feature = "tokio-rt")]
impl<A: Actor> ActorHandle<A::Message> for ActorSpawnHandle<A> {
    fn tell(&self, msg: A::Message) -> BoxFuture<'_, Result<(), MailboxError>> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            tx.send(Message::Msg(msg))
                .await
                .map_err(|_| MailboxError::Closed)
        })
    }
}
