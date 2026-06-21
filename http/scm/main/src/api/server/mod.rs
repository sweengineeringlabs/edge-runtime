//! HTTP server API — traits, types, and errors for the Axum server binding.
mod errors;
mod traits;
mod types;

pub use errors::HttpServerError;
pub use traits::HttpServer;
pub use types::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServerSvc};
