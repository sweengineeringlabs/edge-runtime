//! `DefaultRuntimeManager` — full process lifecycle manager interface.

use crate::api::runtime_manager::RuntimeManager;

/// Marker supertrait for the default process lifecycle manager implementation.
pub trait DefaultRuntimeManager: RuntimeManager {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_runtime_manager_is_object_safe() {
        fn _assert(_: &dyn DefaultRuntimeManager) {}
    }
}
