//! async-std-backed stop handle implementation.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::{Actor, Message, StopHandle};

/// async-std-backed stop handle implementation.
pub(crate) struct AsyncStdStopHandle<A: Actor> {
    pub(crate) tx: Arc<async_std::channel::Sender<Message<A>>>,
}

impl<A: Actor> Clone for AsyncStdStopHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

impl<A: Actor> StopHandle for AsyncStdStopHandle<A> {
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

    /// @covers: AsyncStdStopHandle::stop
    #[async_std::test]
    async fn test_async_std_stop_handle_stop_sends_stop_message() {
        let (tx, rx) = async_std::channel::bounded::<Message<TestActor>>(1);
        let handle: AsyncStdStopHandle<TestActor> = AsyncStdStopHandle { tx: Arc::new(tx) };

        handle.stop().await;

        // Verify stop signal was sent
        let msg = rx.recv().await;
        assert!(matches!(msg, Ok(Message::Stop)));
    }
}
