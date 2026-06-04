//! Application configuration builder.

pub(crate) struct ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "SEA core/ anchor — wired up when config integrates into factory"
        )
    )]
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
