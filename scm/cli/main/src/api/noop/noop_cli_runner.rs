//! [`NoopCliRunner`] — no-op test stub declared in `api/` per SEA `pub_types_in_api_only`.

/// A no-op [`crate::api::CliRunner`] that returns an empty success output.
///
/// Intended for tests and as a placeholder in composition roots before a real
/// CLI runner is wired in.
pub struct NoopCliRunner;
