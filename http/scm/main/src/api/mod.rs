//! Public HTTP runtime contract surface.
//!
//! Port types (HttpIngress, HttpRequest, etc.) are provided by `swe-edge-ingress-http`.
//! Server binding types live in `server/`. Noop stubs live in `noop/`.

mod noop;
mod server;
mod tls;

#[cfg(test)]
mod tests;

pub use noop::{NoopHttpIngress, NoopValidator};
pub use server::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServer, HttpServerError,
    HttpServerSvc,
};
pub use tls::TlsSvc;
