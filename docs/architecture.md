# Runtime Workspace Architecture

Three independent crates, no mandatory cross-dependencies:

```
runtime/
├── main/features/server/        swe-edge-runtime-server
│     optional deps: scheduler, message-broker
├── main/features/scheduler/     swe-edge-runtime-scheduler
│     no server dep
└── main/features/message-broker/ swe-edge-runtime-message-broker
      no server dep
```

Consumers opt in via feature flags on the server crate or depend directly on the scheduler / message-broker crates.
