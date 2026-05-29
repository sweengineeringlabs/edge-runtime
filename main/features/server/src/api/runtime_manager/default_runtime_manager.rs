//! `DefaultRuntimeManager` — full process lifecycle manager interface.

use crate::api::runtime_manager::RuntimeManager;

/// Marker supertrait for the default process lifecycle manager implementation.
pub trait DefaultRuntimeManager: RuntimeManager {}
