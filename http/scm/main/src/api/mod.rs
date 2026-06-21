//! Public HTTP runtime contract surface.
//!
//! Port types (HttpIngress, HttpRequest, etc.) are provided by `swe-edge-ingress-http`.
//! Server binding types live in `server/`. Noop stubs live in `noop/`.

pub mod noop;
pub mod server;
