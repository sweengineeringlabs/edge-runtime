# swe-edge-message-broker

Runtime-agnostic cross-process pub/sub broker for `swe-edge` services. Ships an
in-memory tokio broadcast backend (`tokio-rt`), a NATS backend (`nats`), and
a Kafka backend (`kafka`). Bring your own backend by implementing `MessageBroker`.
