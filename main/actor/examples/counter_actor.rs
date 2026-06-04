//! Counter actor example — demonstrates basic actor usage.
//!
//! This example shows:
//! - Defining an actor with state (Counter)
//! - Defining message types (CounterMessage)
//! - Spawning an actor via SAF factory
//! - Using tell (fire-and-forget) semantics
//! - Request-reply with explicit oneshot channels
//! - Graceful shutdown via StopHandle

use futures::future::BoxFuture;
use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime, StopHandle};

#[derive(Clone)]
struct Counter {
    count: i32,
}

enum CounterMessage {
    Increment,
    #[allow(dead_code)]
    Decrement,
    GetCount(tokio::sync::oneshot::Sender<i32>),
}

impl Actor for Counter {
    type Message = CounterMessage;

    fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            match msg {
                CounterMessage::Increment => {
                    self.count += 1;
                    println!("Incremented: count = {}", self.count);
                }
                CounterMessage::Decrement => {
                    self.count -= 1;
                    println!("Decremented: count = {}", self.count);
                }
                CounterMessage::GetCount(tx) => {
                    println!("GetCount: returning {}", self.count);
                    let _ = tx.send(self.count);
                }
            }
        })
    }

    fn on_stop(&mut self) -> BoxFuture<'_, ()> {
        let count = self.count;
        Box::pin(async move {
            println!("Counter stopping. Final count: {}", count);
        })
    }
}

#[tokio::main]
async fn main() {
    let counter = Counter { count: 0 };
    let (handle, stop) = ActorRuntime::spawn_with_stop(counter);

    // Fire-and-forget: tell()
    println!("Sending increment messages...");
    handle
        .tell(CounterMessage::Increment)
        .await
        .unwrap_or_else(|_| panic!("tell failed"));
    handle
        .tell(CounterMessage::Increment)
        .await
        .unwrap_or_else(|_| panic!("tell failed"));
    handle
        .tell(CounterMessage::Increment)
        .await
        .unwrap_or_else(|_| panic!("tell failed"));

    // Give messages time to process
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Request-reply: send a message with a reply channel
    println!("\nRequesting count...");
    let (tx, rx) = tokio::sync::oneshot::channel();
    handle
        .tell(CounterMessage::GetCount(tx))
        .await
        .unwrap_or_else(|_| panic!("tell failed"));
    let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
    println!("Final count: {}", count);

    // Graceful shutdown
    println!("\nShutting down actor...");
    stop.stop().await;
    println!("Actor stopped.");
}
