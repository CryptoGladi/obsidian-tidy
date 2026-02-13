use crate::rule::Rule;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ToggleableRule<R>
where
    R: Rule,
{
    rule: R,
    enabled: bool,
}

impl<R> ToggleableRule<R>
where
    R: Rule,
{
    pub fn new(rule: R, enabled: bool) -> Self {
        Self { rule, enabled }
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

impl<R> Deref for ToggleableRule<R>
where
    R: Rule,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.rule
    }
}

impl<R> PartialEq for ToggleableRule<R>
where
    R: Rule + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        (&self.rule, self.enabled) == (&other.rule, other.enabled)
    }
}

impl<L> Eq for ToggleableRule<L> where L: Rule + PartialEq {}

#[cfg(test)]
mod tests {
    use crate::rule::{Category, ToggleableRule};
    use crate::test_utils::TestRule;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let rule = Arc::new(TestRule::new("TestRule", "", Category::Content));
        let rule_enabled = ToggleableRule::new(rule.clone(), true);
        let rule_disabled = ToggleableRule::new(rule, false);

        assert!(rule_enabled.enabled());
        assert!(rule_disabled.disabled());
    }

    #[test]
    #[traced_test]
    fn enable() {
        let rule = Arc::new(TestRule::new("TestRule", "", Category::Content));
        let mut rule = ToggleableRule::new(rule, false);

        assert!(rule.disabled());
        rule.enable();
        assert!(rule.enabled());
    }

    #[test]
    #[traced_test]
    fn disable() {
        let rule = Arc::new(TestRule::new("TestRule", "", Category::Content));
        let mut rule = ToggleableRule::new(rule, true);

        assert!(rule.enabled());
        rule.disable();
        assert!(rule.disabled());
    }
}
