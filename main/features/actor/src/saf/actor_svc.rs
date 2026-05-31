//! SAF — actor spawn factories and config builder.

use crate::api::{Actor, ActorHandle, ActorRuntime, StopHandle};

#[cfg(feature = "tokio-rt")]
use crate::api::{ActorContext, Message};

#[cfg(feature = "tokio-rt")]
use std::sync::Arc;

#[cfg(feature = "tokio-rt")]
use tokio::sync::mpsc;

#[cfg(feature = "tokio-rt")]
use crate::api::types::actor::ActorSpawnHandle;

#[cfg(feature = "tokio-rt")]
use crate::spi::tokio::{TokioActorHandle, TokioStopHandle};

/// Bounded channel capacity for actor mailboxes.
#[cfg(feature = "tokio-rt")]
const MAILBOX_CAPACITY: usize = 1024;

impl ActorRuntime {
    /// Spawn an actor with no lifecycle management.
    ///
    /// The actor runs in a spawned tokio task, processing messages sequentially.
    /// Returns a handle to send messages; the actor stops when all handles are dropped.
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    pub fn spawn<A: Actor>(actor: A) -> ActorSpawnHandle<A> {
        let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
        let tx = Arc::new(tx);
        tokio::spawn(Self::run_actor_loop(actor, rx));
        ActorSpawnHandle { tx }
    }

    /// Spawn an actor with explicit lifecycle management.
    ///
    /// Returns both a message handle and a stop handle. Calling `stop()` signals the
    /// actor to shut down gracefully after processing the current message.
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    pub fn spawn_with_stop<A: Actor>(actor: A) -> (impl ActorHandle<A::Message>, impl StopHandle) {
        let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
        let tx = Arc::new(tx);
        tokio::spawn(Self::run_actor_loop(actor, rx));

        let handle = TokioActorHandle {
            tx: Arc::clone(&tx),
        };
        let stop = TokioStopHandle { tx };
        (handle, stop)
    }

    /// The actor's main message processing loop (tokio backend).
    ///
    /// Processes messages sequentially; calls `on_stop` on graceful shutdown.
    #[cfg(feature = "tokio-rt")]
    async fn run_actor_loop<A: Actor>(mut actor: A, mut rx: mpsc::Receiver<Message<A>>) {
        let ctx = ActorContext::new();

        while let Some(msg) = rx.recv().await {
            match msg {
                Message::Msg(m) => {
                    actor.handle(ctx.clone(), m).await;
                }
                Message::Stop => {
                    actor.on_stop().await;
                    break;
                }
            }
        }

        // Channel dropped without Stop signal — run cleanup anyway
        actor.on_stop().await;
    }

    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    ///
    /// This is the primary entry point for loading `config/application.toml`.
    /// The returned builder resolves XDG config directories and layers the
    /// application config file automatically.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }
}
