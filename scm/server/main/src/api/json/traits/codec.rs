//! `Codec` — JSON codec interface for HTTP and gRPC routes.

use crate::api::json::types::json_codec::JsonCodec;

/// Marker trait for JSON codec implementations.
pub trait Codec: JsonCodec {}
