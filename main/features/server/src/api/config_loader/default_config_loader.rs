//! `DefaultConfigLoader` — filesystem-backed layered config loader interface.

use crate::api::config_loader::ConfigLoader;

/// Marker supertrait for filesystem-backed, layered config loaders.
pub trait DefaultConfigLoader: ConfigLoader + Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_loader_is_object_safe() {
        fn _assert(_: &dyn DefaultConfigLoader) {}
    }
}
