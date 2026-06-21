//! Multipart form part.

use serde::{Deserialize, Serialize};

/// A part of a multipart/form-data upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPart {
    /// Form field name.
    pub name: String,
    /// File name when the part is a file upload.
    pub filename: Option<String>,
    /// MIME type of this part.
    pub content_type: Option<String>,
    /// Raw bytes of this part.
    pub data: Vec<u8>,
}
