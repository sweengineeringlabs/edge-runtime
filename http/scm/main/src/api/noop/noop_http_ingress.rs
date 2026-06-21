//! [`NoopHttpIngress`] — no-op test stub declared in `api/` per SEA `pub_types_in_api_only`.

/// A no-op [`crate::api::HttpIngress`] that responds with `200 OK`.
///
/// Intended for tests and as a placeholder in composition roots before a real
/// ingress handler is wired in.
pub struct NoopHttpIngress;
