//! `RuntimeBuilderServe` — facade type for the serve operation.

/// Marker type for the `RuntimeBuilder::serve()` implementation module.
pub struct RuntimeBuilderServe;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_builder_serve_is_send() {
        fn _assert_send<T: Send>() {}
        _assert_send::<RuntimeBuilderServe>();
    }
}
