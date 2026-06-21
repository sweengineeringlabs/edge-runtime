//! RuntimeBuilderServe interface — mirrors `core/runtime/runtime_builder_serve`.

pub use crate::api::runtime::types::runtime_builder_serve::RuntimeBuilderServe;

/// Default port the serve builder binds the HTTP transport to.
pub const DEFAULT_SERVE_HTTP_PORT: u16 = 8080;
