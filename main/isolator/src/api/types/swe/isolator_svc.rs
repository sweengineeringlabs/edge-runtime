//! `IsolatorSvc` — public facade type for the isolation profile subsystem.

/// SAF facade for the isolation-profile subsystem.
///
/// All construction entry points for isolation profiles, registry,
/// and the primary `SweEdgeRuntimeIsolator` service are implemented
/// as associated functions on this type in `saf/`.
pub struct IsolatorSvc;
