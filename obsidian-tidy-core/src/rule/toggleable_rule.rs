use crate::{
    Note,
    rule::{Category, Content, Rule, Violation},
};
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
    pub const fn new(rule: R, enabled: bool) -> Self {
        Self { rule, enabled }
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        !self.enabled
    }

    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    pub const fn disable(&mut self) {
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

/// Impl [`Rule`] for `ToggleableRule`
impl<R> Rule for ToggleableRule<R>
where
    R: Rule,
{
    type Error = R::Error;

    #[inline]
    fn name(&self) -> &str {
        self.deref().name()
    }

    #[inline]
    fn description(&self) -> &str {
        self.deref().description()
    }

    #[inline]
    fn category(&self) -> Category {
        self.deref().category()
    }

    /// If lint is enabled, then run check
    fn check(&self, content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error> {
        if self.is_enabled() {
            return self.deref().check(content, note);
        }

        Ok(Vec::new())
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
    use crate::Note;
    use crate::rule::{Category, Content, Rule, ToggleableRule, Violation};
    use crate::test_utils::TestRule;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let rule = Arc::new(TestRule::new("TestRule", "", Category::Content, []));
        let rule_enabled = ToggleableRule::new(rule.clone(), true);
        let rule_disabled = ToggleableRule::new(rule, false);

        assert!(rule_enabled.is_enabled());
        assert!(rule_disabled.is_disabled());
    }

    #[test]
    #[traced_test]
    fn enable() {
        let rule = TestRule::new("TestRule", "", Category::Content, []);
        let mut rule = ToggleableRule::new(rule, false);

        assert!(rule.is_disabled());
        rule.enable();
        assert!(rule.is_enabled());
    }

    #[test]
    #[traced_test]
    fn disable() {
        let rule = TestRule::new("TestRule", "", Category::Content, []);
        let mut rule = ToggleableRule::new(rule, true);

        assert!(rule.is_enabled());
        rule.disable();
        assert!(rule.is_disabled());
    }

    #[test]
    #[traced_test]
    fn check_enabled() {
        let violation = vec![Violation::new("Super error", 1..2).unwrap()];

        let rule = TestRule::new("test-rule", "", Category::Other, violation.clone());
        let note = Note::default();
        let content = Content::default();

        let rule_enable = ToggleableRule::new(rule, true);
        let result = rule_enable.check(&content, &note).unwrap();

        assert_eq!(result, violation);
    }

    #[test]
    #[traced_test]
    fn check_disabled() {
        let violation = vec![Violation::new("Super error", 1..2).unwrap()];

        let rule = TestRule::new("test-rule", "", Category::Other, violation.clone());
        let note = Note::default();
        let content = Content::default();

        let rule_enable = ToggleableRule::new(rule, false);
        let result = rule_enable.check(&content, &note).unwrap();

        assert!(result.is_empty());
    }
}
