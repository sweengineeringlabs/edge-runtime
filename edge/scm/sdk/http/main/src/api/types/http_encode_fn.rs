//! Encode function type alias for HTTP adapters.

use super::http_response::HttpResponse;

/// Encodes a typed response value into an [`HttpResponse`].
pub type HttpEncodeFn<Resp> = fn(Resp) -> HttpResponse;
