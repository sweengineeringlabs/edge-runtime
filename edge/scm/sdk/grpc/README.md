# swe-edge-runtime-grpc

gRPC ingress contract crate for the swe-edge runtime layer.

Provides `GrpcIngress` trait and supporting value types. Contains no transport
dependencies (no Tonic, no protobuf codegen). Plugins and transport crates both
implement `GrpcIngress`; the composition root wires them.

## Usage

Add to `Cargo.toml`:

```toml
swe-edge-runtime-grpc = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", tag = "v0.3.9" }
```

## Structure

- `api/` — `GrpcIngress` trait + all value types
- `core/` — `NoopGrpcIngress` implementation
- `saf/` — public re-export surface (consumers only import from here)
- `spi/` — extension hooks
