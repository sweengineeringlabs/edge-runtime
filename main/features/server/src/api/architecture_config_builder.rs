/// Builder for architecture configuration.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ArchitectureConfigBuilder {
    _private: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_config_builder_constructs() {
        let _b = ArchitectureConfigBuilder::default();
    }
}
