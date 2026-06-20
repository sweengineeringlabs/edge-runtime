# RFC-001 — ZeroMQ Broker Backend

**Status:** Proposed  
**Date:** 2026-06-17  
**Scope:** `swe-edge-runtime-message-broker` — new `zmq` feature flag adding a ZeroMQ SPI backend

---

## Motivation

The current message-broker crate ships two backends — `nats` and `kafka`. A third backend based on ZeroMQ is requested for deployments where:

- **No broker process is wanted** — ZeroMQ is brokerless; there is no server to start, monitor, or restart. This simplifies single-host and embedded deployments.
- **Raw latency matters** — ZeroMQ operates at ~10 µs round-trip vs ~100 µs for NATS in loopback benchmarks. For high-frequency data pipelines (e.g. 32-bit DLL → ML consumer on the same machine) this headroom is meaningful.
- **Footprint** — no sidecar process, no port to expose, no monitoring endpoint to secure.

---

## Proposed change

Add a `zmq` feature flag to `swe-edge-runtime-message-broker` that wires a ZeroMQ PUB/SUB pair behind the existing `MessageBroker` trait. The public API surface is unchanged.

### Factory method

```rust
// existing pattern — unchanged
MessageBrokerFactory::nats("nats://localhost:4222").await?;
MessageBrokerFactory::kafka("localhost:9092").await?;

// proposed
MessageBrokerFactory::zmq("tcp://localhost:5555")?;  // synchronous — no broker handshake
```

### Feature flag

```toml
# Cargo.toml
[features]
zmq = ["zeromq"]                 # pure-Rust — no C dep, no libzmq install required

[dependencies]
zeromq = { version = "0.4", optional = true }
```

`zeromq` (pure Rust) is preferred over the `zmq` crate (wraps libzmq) to avoid a C library dependency and keep the i686/cross-compile story clean.

### SPI placement

Following the existing pattern:

```
spi/broker/
  nats/          ← existing
  kafka/         ← existing
  zmq/           ← new
    mod.rs
    zmq_message_broker.rs
```

### Transport semantics

| Concern | Decision |
|---|---|
| Pattern | PUB/SUB — matches NATS topic semantics; publisher does not block on consumer readiness |
| Topic routing | ZeroMQ topic-prefix filtering on the SUB socket — subject string sent as frame prefix |
| Endpoint | `tcp://host:port` — same format as NATS URL, familiar to operators |
| Reconnect | ZeroMQ handles reconnect internally; no application-layer retry needed |
| Async | `zeromq` crate is async-native (tokio-compatible) |

### Config

```toml
# application.toml / consumer config
[broker]
backend = "zmq"
url     = "tcp://localhost:5555"
```

---

## Alternatives considered

| Option | Verdict |
|---|---|
| `zmq` crate (libzmq binding) | Rejected — requires libzmq installed; breaks i686 cross-compile without extra toolchain setup |
| NATS (status quo) | Good for multi-host, monitoring, persistence. Kept. ZMQ is additive, not a replacement. |
| Kafka | Heavy — requires a running cluster. Not suitable for local/embedded use. |
| Unix domain sockets (custom) | Would need custom framing, reconnect, and pub/sub logic from scratch. ZMQ provides all of this. |

---

## Consequences

**Positive**
- Lower latency path for same-host deployments (DLL → ML pipeline)
- No broker process to manage — simpler ops for local/dev environments
- Additive: existing NATS and Kafka backends unchanged
- Pure-Rust dep — no C toolchain requirement, cross-compile friendly

**Negative**
- No persistence — messages in flight are lost if the subscriber is not connected (fire-and-forget only)
- No built-in monitoring endpoint (unlike NATS `-m 8222`)
- ZMQ PUB drops messages silently if no subscriber is connected — acceptable for tick streaming, not for order submission

**Constraints**
- Order submission (`TaskQueue`) should NOT use the ZMQ backend in production — silent drop on no-subscriber is unsafe for orders. The backend should document this clearly or gate order use behind a compile error.

---

## Follow-ups

- [ ] Implement `ZmqMessageBroker` in `spi/broker/zmq/`
- [ ] Add `MessageBrokerFactory::zmq(endpoint)` to `saf/`
- [ ] Integration test: `zmq_message_broker_int_test.rs`
- [ ] Update `config/application.toml` with `zmq` backend example
- [ ] Document the no-persistence / silent-drop constraint in the broker trait doc
