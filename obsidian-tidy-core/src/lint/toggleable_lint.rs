use crate::lint::DynLint;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ToggleableLint<E>
where
    E: std::error::Error,
{
    lint: DynLint<E>,
    enabled: bool,
}

impl<E> ToggleableLint<E>
where
    E: std::error::Error,
{
    pub fn new(lint: DynLint<E>, enabled: bool) -> Self {
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

impl<E> Deref for ToggleableLint<E>
where
    E: std::error::Error,
{
    type Target = DynLint<E>;

    fn deref(&self) -> &Self::Target {
        &self.lint
    }
}

impl<E> PartialEq for ToggleableLint<E>
where
    E: std::error::Error,
{
    fn eq(&self, other: &Self) -> bool {
        (&self.lint, self.enabled) == (&other.lint, other.enabled)
    }
}

impl<E> Eq for ToggleableLint<E> where E: std::error::Error {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::lint::{Category, ToggleableLint};
    use crate::test_utils::TestLint;

    #[test]
    fn new() {
        let lint = Arc::new(TestLint::new("TestLint", "", Category::Content));
        let lint = ToggleableLint::new(lint, true);

        assert!(lint.enabled())
    }
}
