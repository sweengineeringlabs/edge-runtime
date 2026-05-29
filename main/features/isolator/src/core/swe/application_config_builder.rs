//! Application configuration builder.

pub(crate) struct ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_config_builder_new() {
        let _ = ApplicationConfigBuilder::new();
    }
}
