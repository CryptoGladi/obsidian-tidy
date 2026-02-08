use super::Lint;
use std::ops::Deref;

#[derive(Debug)]
pub struct ToggleableLint {
    lint: Box<dyn Lint>,
    enabled: bool,
}

impl ToggleableLint {
    pub fn new(lint: Box<dyn Lint>) -> Self {
        Self {
            lint,
            enabled: true,
        }
    }

    pub fn with_enabled(lint: Box<dyn Lint>, enabled: bool) -> Self {
        Self { lint, enabled }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Deref for ToggleableLint {
    type Target = Box<dyn Lint>;

    fn deref(&self) -> &Self::Target {
        &self.lint
    }
}
