use swe_edge_runtime_message_broker::broker_from_config;

/// @covers: broker_from_config
#[tokio::test]
#[cfg(feature = "tokio-rt")]
async fn test_broker_from_config_loads_and_instantiates_default_inmemory() {
    let broker = broker_from_config()
        .await
        .expect("broker_from_config failed");
    assert!(broker.health_check().await.is_ok());
}
