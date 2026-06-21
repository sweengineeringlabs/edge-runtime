//! SAF re-exports for HTTP server types.

pub use crate::api::server::error::HttpServerError;
pub use crate::api::server::traits::HttpServer;
pub use crate::api::server::types::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper};

/// Slug identifying the server SAF service.
pub const HTTP_SERVER_SVC: &str = "http_server";
