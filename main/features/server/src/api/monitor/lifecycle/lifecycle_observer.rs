/// Marker trait for [`LifecycleMonitor`](edge_proxy::LifecycleMonitor) wrappers
/// that emit observability signals (metrics, traces) on health transitions.
pub trait LifecycleObserver: edge_proxy::LifecycleMonitor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_observer_is_object_safe() {
        fn _accept(_: &dyn LifecycleObserver) {}
    }
}
