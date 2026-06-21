//! No-op [`crate::api::CliCommand`] implementation for tests.

use std::collections::HashMap;

/// A test-only [`crate::api::CliCommand`] that carries an arbitrary name and arg bag.
///
/// Stores raw primitives so it can live in `api/` without concrete-struct field violations.
/// Construct via [`crate::NoopCliCommand::create`] or [`crate::NoopCliCommand::create_with_args`].
pub struct NoopCliCommand {
    pub(crate) name: String,
    pub(crate) positional: Vec<String>,
    pub(crate) flags: HashMap<String, String>,
}
