//! Tokio-backed stop handle implementation.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::api::{Actor, Message, StopHandle};

/// Tokio-backed stop handle implementation.
pub(crate) struct TokioStopHandle<A: Actor> {
    pub(crate) tx: Arc<mpsc::Sender<Message<A>>>,
}

impl<A: Actor> Clone for TokioStopHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

impl<A: Actor> StopHandle for TokioStopHandle<A> {
    fn stop(&self) -> BoxFuture<'_, ()> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            let _ = tx.send(Message::Stop).await;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestActor;

    impl Actor for TestActor {
        type Message = ();

        fn handle(
            &mut self,
            _ctx: crate::api::ActorContext<Self>,
            _msg: Self::Message,
        ) -> BoxFuture<'_, ()> {
            Box::pin(async move {})
        }
    }

    /// @covers: TokioStopHandle::stop
    #[tokio::test]
    async fn test_tokio_stop_handle_stop_sends_stop_message() {
        let (tx, mut rx) = mpsc::channel::<Message<TestActor>>(1);
        let handle: TokioStopHandle<TestActor> = TokioStopHandle { tx: Arc::new(tx) };

        handle.stop().await;

        // Verify stop signal was sent
        let msg = rx.recv().await;
        assert!(matches!(msg, Some(Message::Stop)));
    }
}
