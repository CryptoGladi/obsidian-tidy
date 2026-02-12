use crate::lint::Lint;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ToggleableLint<L>
where
    L: Lint,
{
    lint: L,
    enabled: bool,
}

impl<L> ToggleableLint<L>
where
    L: Lint,
{
    pub fn new(lint: L, enabled: bool) -> Self {
        Self { lint, enabled }
    }

    #[must_use]
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
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

impl<L> Deref for ToggleableLint<L>
where
    L: Lint,
{
    type Target = L;

    fn deref(&self) -> &Self::Target {
        &self.lint
    }
}

impl<L> PartialEq for ToggleableLint<L>
where
    L: Lint + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (&self.lint, self.enabled) == (&other.lint, other.enabled)
    }
}

impl<L> Eq for ToggleableLint<L> where L: Lint + PartialEq {}

#[cfg(test)]
mod tests {
    use crate::lint::{Category, ToggleableLint};
    use crate::test_utils::TestLint;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let lint = Arc::new(TestLint::new("TestLint", "", Category::Content));
        let lint_enabled = ToggleableLint::new(lint.clone(), true);
        let lint_disabled = ToggleableLint::new(lint, false);

        assert!(lint_enabled.enabled());
        assert!(lint_disabled.disabled());
    }

    #[test]
    #[traced_test]
    fn enable() {
        let lint = Arc::new(TestLint::new("TestLint", "", Category::Content));
        let mut lint = ToggleableLint::new(lint, false);

        assert!(lint.disabled());
        lint.enable();
        assert!(lint.enabled());
    }

    #[test]
    #[traced_test]
    fn disable() {
        let lint = Arc::new(TestLint::new("TestLint", "", Category::Content));
        let mut lint = ToggleableLint::new(lint, true);

        assert!(lint.enabled());
        lint.disable();
        assert!(lint.disabled());
    }
}
