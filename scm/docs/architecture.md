# Architecture — edge-runtime

Two crates: `swe-edge-runtime-server` (HTTP/gRPC runtime assembly) and `swe-edge-runtime-message-broker` (broker backends: in-memory, NATS, Kafka).

---

## Sequence — Runtime Server

> `RuntimeBuilder` is constructed and configured fluently; `serve()` binds ports and blocks until shutdown signal.

```mermaid
sequenceDiagram
    participant App
    participant RuntimeBuilder
    participant ServerSvc
    participant HttpServer
    participant GrpcServer
    participant LifecycleMonitor

    App->>RuntimeBuilder: new()
    App->>RuntimeBuilder: http_route(handler)
    App->>RuntimeBuilder: grpc_route(handler)
    App->>RuntimeBuilder: http_tls(tls_config)
    App->>RuntimeBuilder: http_bearer_auth(jwt_verifier)
    App->>RuntimeBuilder: lifecycle(monitor)

    App->>RuntimeBuilder: serve()
    RuntimeBuilder->>ServerSvc: build(config)
    ServerSvc->>LifecycleMonitor: on_start()
    ServerSvc->>HttpServer: bind(http_addr)
    ServerSvc->>GrpcServer: bind(grpc_addr)
    ServerSvc->>ServerSvc: await SIGTERM / Ctrl+C
    ServerSvc->>LifecycleMonitor: on_stop()
    ServerSvc-->>App: Result<(), RuntimeError>
```

## Data Flow — Runtime Server

> Config + handlers enter `RuntimeBuilder`; the assembled runtime exposes three ports and terminates gracefully on signal.

```mermaid
flowchart LR
    A["RuntimeConfig\n───────────\nhttp_bind: SocketAddr\ngrpc_bind: SocketAddr\nmetrics_bind: SocketAddr\nshutdown_timeout_secs"] --> B["RuntimeBuilder"]
    C["Handlers\nArc<dyn Handler>"] --> B
    D["Middleware\nTokenVerifier\nIngressTlsConfig\nLifecycleMonitor"] --> B

    B --> E["ServerSvc::serve"]
    E --> F["HTTP :8080\n(axum)"]
    E --> G["gRPC :50051\n(tonic)"]
    E --> H["Metrics :9090\n(Prometheus)"]

    I["SIGTERM / Ctrl+C"] --> E
    E -->|graceful drain| J["Result<(), RuntimeError>"]
```

---

## Sequence — Message Broker Backend

> `BrokerSvc::from_config` reads the active backend from TOML and wires it; `publish` and `subscribe` are symmetric.

```mermaid
sequenceDiagram
    participant App
    participant BrokerSvc
    participant MessageBroker
    participant Backend

    App->>BrokerSvc: from_config(broker_config)
    BrokerSvc->>BrokerSvc: match backend_kind (InMemory | Nats | Kafka)
    BrokerSvc-->>App: Arc<dyn MessageBroker>

    App->>MessageBroker: publish(topic, message)
    MessageBroker->>Backend: send bytes
    Backend-->>MessageBroker: ack
    MessageBroker-->>App: Result<(), BrokerError>

    App->>MessageBroker: subscribe(topic)
    MessageBroker->>Backend: open subscription
    Backend-->>MessageBroker: MessageStream
    MessageBroker-->>App: MessageStream (async iterator)
```

## Data Flow — Message Broker Backend

> A `MessageBrokerConfig` selects and constructs the backend; messages flow through the backend and emerge as a `MessageStream`.

```mermaid
flowchart LR
    A["MessageBrokerConfig\n───────────\nbackend: BackendKind\nnats_url / kafka_brokers\ntopic_prefix"] --> B["BrokerSvc::from_config"]
    B --> C{BackendKind}
    C -->|InMemory| D["tokio broadcast\n(in-process)"]
    C -->|Nats| E["async-nats\nclient + subject"]
    C -->|Kafka| F["rdkafka\nproducer + consumer"]

    G["Message\n───────────\ntopic: String\npayload: Bytes\nheaders: HashMap"] --> D
    G --> E
    G --> F

    D --> H["MessageStream\n(async_stream)"]
    E --> H
    F --> H
```
