# swe-edge-runtime-http

HTTP ingress/egress contract crate for the swe-edge runtime layer.

Provides `HttpIngress` trait and supporting value types. Contains no transport
dependencies (no Axum, no TLS, no connection-pool). Plugins and transport
crates both implement `HttpIngress`; the composition root wires them.

## Usage

Add to `Cargo.toml`:

```toml
swe-edge-runtime-http = { git = "https://github.com/sweengineeringlabs/edge-runtime.git", tag = "v0.3.9" }
```

## Structure

- `api/` — `HttpIngress` trait + all value types
- `core/` — `NoopHttpIngress` implementation
- `saf/` — public re-export surface (consumers only import from here)
- `spi/` — extension hooks
