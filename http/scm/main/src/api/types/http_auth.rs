//! HTTP authentication credential types.

use serde::{Deserialize, Serialize};

/// Authentication credential extracted from an inbound HTTP request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HttpAuth {
    /// No authentication credential present.
    #[default]
    None,
    /// Bearer token (`Authorization: Bearer <token>`).
    Bearer {
        /// The bearer token value.
        token: String,
    },
    /// HTTP Basic authentication.
    Basic {
        /// Username.
        username: String,
        /// Password.
        password: String,
    },
    /// API key via a custom request header.
    ApiKey {
        /// Header name carrying the key.
        header: String,
        /// API key value.
        key: String,
    },
}

impl HttpAuth {
    /// Construct a Bearer credential.
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer {
            token: token.into(),
        }
    }

    /// Construct a Basic credential.
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Construct an API key credential.
    pub fn api_key(header: impl Into<String>, key: impl Into<String>) -> Self {
        Self::ApiKey {
            header: header.into(),
            key: key.into(),
        }
    }
}
