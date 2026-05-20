//! `Sampler` — background metric sampling loop interface.

/// Marker trait for types that run a background metric-sampling loop.
pub trait Sampler: Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_is_object_safe() {
        fn _assert(_: &dyn Sampler) {}
    }
}
