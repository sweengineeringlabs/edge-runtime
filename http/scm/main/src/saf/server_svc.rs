//! SAF re-exports for HTTP server types.

pub use crate::api::HttpServerError;
pub use crate::api::HttpServer;
pub use crate::api::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper};

/// Slug identifying the server SAF service.
pub const HTTP_SERVER_SVC: &str = "http_server";
