//! `Codec` — re-exported from the JSON traits module.

pub use crate::api::json::traits::codec::Codec;

/// Canonical MIME subtype name for this codec.
pub const CODEC_NAME: &str = "json";
