//! Decode function type alias for HTTP adapters.

use crate::api::error::HttpIngressError;
use crate::api::types::http_request::HttpRequest;

/// Decodes a typed request value from an inbound [`HttpRequest`].
pub type HttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpIngressError>;
