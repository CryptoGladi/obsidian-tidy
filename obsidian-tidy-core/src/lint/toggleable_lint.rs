use crate::lint::DynLint;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ToggleableLint {
    lint: DynLint,
    enabled: bool,
}

impl ToggleableLint {
    pub fn new(lint: DynLint, enabled: bool) -> Self {
        Self { lint, enabled }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn disabled(&self) -> bool {
        !self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Deref for ToggleableLint {
    type Target = DynLint;

    fn deref(&self) -> &Self::Target {
        &self.lint
    }
}

impl PartialEq for ToggleableLint {
    fn eq(&self, other: &Self) -> bool {
        (&self.lint, self.enabled) == (&other.lint, other.enabled)
    }
}

impl Eq for ToggleableLint {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::lint::{Category, ToggleableLint};
    use crate::test_utils::TestLint;

    #[test]
    fn new() {
        let lint = Arc::new(TestLint::new("TestLint", Category::Content));
        let lint = ToggleableLint::new(lint, true);

        assert!(lint.enabled())
    }
}
