# Changelog

## 0.1.0 (2026-05-18)
- Initial release: `MessageBroker` trait, `Message`, `BrokerError`, `MessageStream`.
- `InMemoryMessageBroker` (tokio-rt): tokio broadcast-channel backend.
- `NatsMessageBroker` (nats): async-nats pure-Rust backend.
