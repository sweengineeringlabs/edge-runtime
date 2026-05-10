//! `EdgeRuntimeBuilderServe` — facade type for the serve operation.

/// Marker type for the `EdgeRuntimeBuilder::serve()` implementation module.
pub struct EdgeRuntimeBuilderServe;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_runtime_builder_serve_is_send() {
        fn _assert_send<T: Send>() {}
        _assert_send::<EdgeRuntimeBuilderServe>();
    }
}
