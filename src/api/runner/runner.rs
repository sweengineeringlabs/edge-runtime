//! Runner contract — start, await signal, drain.

use crate::api::error::RuntimeResult;
use crate::api::runtime_manager::RuntimeManager;

/// Drives a [`RuntimeManager`] through start → signal → shutdown.
pub trait Runner: Send + Sync {
    fn run(&self) -> RuntimeResult<()>;
}
