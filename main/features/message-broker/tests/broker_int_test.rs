//! Integration tests for [`swe_edge_runtime_message_broker`].

#[cfg(feature = "tokio-rt")]
mod in_memory_tests {
    use bytes::Bytes;
    use futures::StreamExt;
    use swe_edge_runtime_message_broker::{Message, MessageBroker, MessageBrokerFactory};

    /// @covers: MessageBrokerFactory::in_memory
    #[tokio::test]
    async fn test_subscribe_then_publish_roundtrip() {
        let broker = MessageBrokerFactory::in_memory();
        let mut stream = broker
            .subscribe("greetings")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        broker
            .publish("greetings", Message::new(b"hello".as_ref()))
            .await
            .map_err(|e| e.to_string())
            .ok();
        let msg = stream
            .next()
            .await
            .unwrap()
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"hello"));
    }

    #[tokio::test]
    async fn test_publish_with_no_subscribers_succeeds() {
        let broker = MessageBrokerFactory::in_memory();
        let result = broker
            .publish("unsubscribed", Message::new(b"drop".as_ref()))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_messages_delivered_in_order() {
        let broker = MessageBrokerFactory::in_memory();
        let mut stream = broker
            .subscribe("ordered")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        for i in 0u8..5 {
            broker
                .publish("ordered", Message::new(vec![i]))
                .await
                .map_err(|e| e.to_string())
                .ok();
        }
        for expected in 0u8..5 {
            let msg = stream
                .next()
                .await
                .unwrap()
                .map_err(|e| e.to_string())
                .ok()
                .unwrap();
            assert_eq!(
                msg.payload[0], expected,
                "message out of order at index {expected}"
            );
        }
    }

    #[tokio::test]
    async fn test_two_independent_topics_do_not_cross_deliver() {
        let broker = MessageBrokerFactory::in_memory();
        let mut orders = broker
            .subscribe("orders")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        broker
            .publish("payments", Message::new(b"pay".as_ref()))
            .await
            .map_err(|e| e.to_string())
            .ok();
        broker
            .publish("orders", Message::new(b"order".as_ref()))
            .await
            .map_err(|e| e.to_string())
            .ok();
        let msg = orders
            .next()
            .await
            .unwrap()
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"order"));
    }

    #[tokio::test]
    async fn test_clone_handle_shares_channels() {
        let broker = MessageBrokerFactory::in_memory();
        let handle = broker.clone();
        let mut stream = broker
            .subscribe("clone-test")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        handle
            .publish("clone-test", Message::new(b"shared".as_ref()))
            .await
            .map_err(|e| e.to_string())
            .ok();
        let msg = stream
            .next()
            .await
            .unwrap()
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"shared"));
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        assert!(MessageBrokerFactory::in_memory()
            .health_check()
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_message_headers_are_preserved() {
        use std::collections::HashMap;
        let broker = MessageBrokerFactory::in_memory();
        let mut stream = broker
            .subscribe("typed")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        let mut headers = HashMap::new();
        headers.insert("content-type".into(), "application/json".into());
        broker
            .publish("typed", Message::with_headers(b"{}".as_ref(), headers))
            .await
            .map_err(|e| e.to_string())
            .ok();
        let msg = stream
            .next()
            .await
            .unwrap()
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        assert_eq!(
            msg.headers.get("content-type").map(String::as_str),
            Some("application/json")
        );
    }

    #[tokio::test]
    async fn test_fan_out_delivers_to_all_subscribers() {
        let broker = MessageBrokerFactory::in_memory();
        let mut s1 = broker
            .subscribe("fanout")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        let mut s2 = broker
            .subscribe("fanout")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        let mut s3 = broker
            .subscribe("fanout")
            .await
            .map_err(|e| e.to_string())
            .ok()
            .unwrap();
        broker
            .publish("fanout", Message::new(b"broadcast".as_ref()))
            .await
            .map_err(|e| e.to_string())
            .ok();
        for stream in [&mut s1, &mut s2, &mut s3] {
            let msg = stream
                .next()
                .await
                .unwrap()
                .map_err(|e| e.to_string())
                .ok()
                .unwrap();
            assert_eq!(msg.payload, Bytes::from_static(b"broadcast"));
        }
    }
}
