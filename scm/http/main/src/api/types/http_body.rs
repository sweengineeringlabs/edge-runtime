//! HTTP request body variants.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::form_part::FormPart;

/// HTTP request body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HttpBody {
    /// JSON body.
    Json(serde_json::Value),
    /// Raw bytes body.
    Raw(Vec<u8>),
    /// URL-encoded form body.
    Form(HashMap<String, String>),
    /// Multipart form body.
    Multipart(Vec<FormPart>),
}
