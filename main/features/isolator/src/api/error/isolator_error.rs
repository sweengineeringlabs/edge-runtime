//! `IsolatorError` — domain error alias for swe_edge_runtime_isolator.

/// Domain error alias — the canonical error type for this crate.
#[expect(dead_code, reason = "SEA api/ anchor — exported for consumers, not used internally")]
pub type IsolatorError = crate::api::error::error::Error;
