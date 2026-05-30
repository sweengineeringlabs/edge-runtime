//! Example: in-memory pub/sub broker.
//!
//! Demonstrates how to use [`swe_edge_runtime_message_broker`] with the `tokio-rt`
//! feature to pub/sub messages in the same process.

#[cfg(feature = "tokio-rt")]
#[tokio::main]
async fn main() {
    use futures::StreamExt as _;
    use swe_edge_runtime_message_broker::{Message, MessageBroker as _, MessageBrokerFactory};

    let broker = MessageBrokerFactory::in_memory();

    // Subscribe before publishing to avoid missing messages.
    let mut stream = broker.subscribe("events").await.unwrap();

    broker
        .publish("events", Message::new(b"hello from producer".as_ref()))
        .await
        .unwrap();

    let msg = stream.next().await.unwrap().unwrap();
    println!("Received: {}", String::from_utf8_lossy(&msg.payload));
}

#[cfg(not(feature = "tokio-rt"))]
fn main() {
    eprintln!("This example requires the `tokio-rt` feature. Run with --features tokio-rt.");
}
