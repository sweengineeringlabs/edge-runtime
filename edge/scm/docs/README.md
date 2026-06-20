# swe-edge-runtime Overview

## WHAT

`swe-edge-runtime` is the process-level runtime for `swe-edge` — wires ingress, proxy, domain handlers, egress, and lifecycle into a single deployable server with an optional message-broker extension.

### server crate (`swe-edge-runtime-server`)

| Capability | Type | Description |
|------------|------|-------------|
| `RuntimeBuilder` | builder | Fluent API for registering HTTP/gRPC routes, configuring TLS, bearer auth, egress clients, and lifecycle hooks before `serve()` |
| `RuntimeConfig` | value object | Typed TOML configuration loaded via XDG — bind addresses, TLS, metrics, tracing, message-broker |
| `RuntimeManager` | type | Manages process lifecycle: start, health-check, graceful shutdown, SIGTERM handling |
| `ServerSvc` | type | Wires `AxumHttpServer` + `TonicGrpcServer` from config, resolves verifier/TLS/reflections |
| `HttpLoadMonitor` | decorator | Wraps any `HttpIngress` handler; records in-flight count, total requests, and latency histograms |
| `MetricsHandler` | type | Serves Prometheus text exposition on `GET /metrics` from a shared `MetricsProvider` |
| `JsonCodec` | type | `grpc_json_decode`/`grpc_json_encode` and `json_decode`/`json_encode` for typed HTTP/gRPC codec wiring |
| `ServiceRegistry` | type | Thread-safe registry mapping service-id strings to `Arc<dyn HttpIngress>` implementations |
| `TrafficCounters` | type | Shared atomic counters (in-flight, total requests, latency ring buffer) backing load-monitor metrics |

### message-broker crate (`swe-edge-runtime-message-broker`)

| Capability | Type | Description |
|------------|------|-------------|
| `MessageBroker` | trait | Backend-agnostic broker — `publish(topic, message)` and `subscribe(topic)` returns a `MessageStream` |
| `Message` / `MessageStream` | value objects | Typed message envelope + async stream of inbound messages |
| `BrokerSvc` | factory | Instantiates the correct backend from `MessageBrokerConfig` — in-memory, NATS, or Kafka |
| `MessageBrokerConfig` | value object | Typed TOML section (`[message-broker]`) — backend selector, topic, buffer size, URLs |
| In-memory backend | type | Zero-dependency `tokio::sync::broadcast`-backed broker for tests and local dev |
| NATS backend | feature | `async_nats`-backed publisher/subscriber (enable feature `nats`) |
| Kafka backend | feature | `rdkafka`-backed producer/consumer (enable feature `kafka`, requires C compiler) |

## WHY

| Problem | Solution |
|---------|----------|
| Wiring HTTP + gRPC ingress, TLS, bearer auth, egress clients, and lifecycle hooks from scratch for every service is repetitive and error-prone | `RuntimeBuilder` provides a single fluent API that assembles the full server from typed config with one `serve()` call |
| Observability (Prometheus metrics, tracing) should be automatic, not bolted on | `HttpLoadMonitor` wraps any handler, `MetricsHandler` serves `/metrics`, and `RuntimeConfig` loads tracing config from TOML |
| Graceful shutdown under SIGTERM requires careful coordination across ingress servers, background tasks, and lifecycle monitors | `RuntimeManager` listens for OS signals, calls `LifecycleMonitor::shutdown()` in order, and drains in-flight requests before exit |
| Message broker backends (in-memory, NATS, Kafka) expose different APIs — swapping backends breaks callers | `MessageBroker` trait + feature-gated `BrokerSvc::from_config()` lets consumers swap backends with a single config change, no code change |
| Runtime config (bind ports, TLS paths, broker URLs) scattered across env vars and code | `RuntimeConfig` maps a single TOML file (XDG-resolved) to a typed struct; `ApplicationConfigBuilder` layers defaults, workspace overrides, and env substitution |
