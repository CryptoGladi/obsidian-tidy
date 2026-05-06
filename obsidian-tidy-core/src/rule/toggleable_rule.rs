//! Rule composition utilities.
//!
//! This module provides [`ToggleableRule`], a wrapper that adds runtime
//! enable/disable control

use crate::rule::{Rule, RuleMetadata, RuleRunner, erased_rule::ErasedRule};
use std::ops::Deref;

/// A wrapper that adds enable/disable toggle functionality to any rule.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use obsidian_tidy_core::rule::{Category, ToggleableRule};
/// use obsidian_tidy_core::test_utils::TestRule;
/// use crate::obsidian_tidy_core::rule::RuleMetadata;
///
/// let rule = TestRule::new("my-rule", "Checks something", Category::Content, []);
/// let mut toggleable = ToggleableRule::new(rule, true);
///
/// assert!(toggleable.is_enabled());
/// assert_eq!(toggleable.name(), "my-rule"); // via Deref to RuleMetadata
///
/// toggleable.disable();
/// assert!(toggleable.is_disabled());
/// ```
///
/// ## Conditional execution via RuleRunner
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Content, RuleRunner, ToggleableRule, Violation};
/// use obsidian_tidy_core::test_utils::TestRule;
/// use obsidian_tidy_core::Note;
///
/// let violations = vec![Violation::new("Error", 0..5).unwrap()];
/// let rule = TestRule::new("strict", "Strict rule", Category::Heading, violations);
/// let note = Note::default();
/// let content = Content::default();
///
/// // Enabled: rule executes and returns violations
/// let enabled = ToggleableRule::new(rule.clone(), true);
/// assert!(!enabled.check(&content, &note).unwrap().is_empty());
///
/// // Disabled: rule is skipped, returns empty
/// let disabled = ToggleableRule::new(rule, false);
/// assert!(disabled.check(&content, &note).unwrap().is_empty());
/// ```
///
/// ## Type erasure for heterogeneous collections
///
/// ```
/// use obsidian_tidy_core::rule::{Category, ToggleableRule, erased_rule::ErasedRule};
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule = TestRule::new("erasable", "Can be erased", Category::Other, []);
/// let toggleable = ToggleableRule::new(rule, true);
///
/// // Convert to type-erased form for storage in Vec<Box<dyn ErasedRule>>
/// let erased: ToggleableRule<Box<dyn ErasedRule>> = toggleable.into_erased();
/// assert_eq!(erased.name(), "erasable");
/// ```
#[derive(Debug, Clone)]
pub struct ToggleableRule<R>
where
    R: RuleMetadata,
{
    /// The underlying rule instance.
    rule: R,

    /// Runtime flag controlling whether the rule should be executed.
    enabled: bool,
}

impl<R> ToggleableRule<R>
where
    R: RuleMetadata,
{
    /// Creates a new `ToggleableRule` wrapping the given rule.
    ///
    /// # Arguments
    ///
    /// * `rule` — The rule to wrap.
    /// * `enabled` — Initial enabled state. `true` means the rule will execute
    ///   when [`check`](RuleRunner::check) is called.
    #[must_use]
    pub const fn new(rule: R, enabled: bool) -> Self {
        Self { rule, enabled }
    }

    /// Returns `true` if the rule is currently enabled.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("check", "Check rule", Category::Content, []);
    /// let toggleable = ToggleableRule::new(rule, true);
    ///
    /// assert!(toggleable.is_enabled());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns `true` if the rule is currently disabled.
    ///
    /// This is the logical negation of [`is_enabled`](Self::is_enabled), provided
    /// for readability in conditional checks.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("skip", "Skip rule", Category::Other, []);
    /// let toggleable = ToggleableRule::new(rule, false);
    ///
    /// assert!(toggleable.is_disabled());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_disabled(&self) -> bool {
        !self.enabled
    }

    /// Enables the rule, allowing it to execute when [`check`](RuleRunner::check) is called.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("reactivate", "Reactivatable", Category::Heading, []);
    /// let mut toggleable = ToggleableRule::new(rule, false);
    ///
    /// assert!(toggleable.is_disabled());
    /// toggleable.enable();
    /// assert!(toggleable.is_enabled());
    /// ```
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the rule, preventing execution in [`check`](RuleRunner::check).
    ///
    /// When disabled, [`check`](RuleRunner::check) returns an empty vector without
    /// invoking the underlying rule's logic — a zero-cost short-circuit.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::obsidian_tidy_core::rule::RuleMetadata;
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("suppress", "Suppressible", Category::Yaml, []);
    /// let mut toggleable = ToggleableRule::new(rule, true);
    ///
    /// assert!(toggleable.is_enabled());
    /// toggleable.disable();
    /// assert!(toggleable.is_disabled());
    /// ```
    pub const fn disable(&mut self) {
        self.enabled = false;
    }

    /// Toggles the enabled state of the rule.
    ///
    /// If the rule is enabled, it becomes disabled, and vice versa.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("flip", "Flippable", Category::Other, []);
    /// let mut toggleable = ToggleableRule::new(rule, true);
    ///
    /// assert!(toggleable.is_enabled());
    /// toggleable.toggle();
    /// assert!(toggleable.is_disabled());
    /// toggleable.toggle();
    /// assert!(toggleable.is_enabled());
    /// ```
    pub const fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Consumes the wrapper and returns the underlying rule.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::obsidian_tidy_core::rule::RuleMetadata;
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("extract", "Extractable", Category::Content, []);
    /// let toggleable = ToggleableRule::new(rule.clone(), false);
    ///
    /// let extracted = toggleable.into_rule();
    /// assert_eq!(extracted.name(), "extract");
    /// ```
    pub fn into_rule(self) -> R {
        self.rule
    }

    /// Returns a reference to the underlying rule.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::obsidian_tidy_core::rule::RuleMetadata;
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("inner", "Inner access", Category::Other, []);
    /// let toggleable = ToggleableRule::new(rule, true);
    ///
    /// // Via Deref (preferred)
    /// assert_eq!(toggleable.name(), "inner");
    ///
    /// // Explicit access
    /// assert_eq!(toggleable.as_rule().name(), "inner");
    /// ```
    ///
    /// # See also
    ///
    /// - [`ToggleableRule::as_rule_mut`]
    #[must_use]
    pub const fn as_rule(&self) -> &R {
        &self.rule
    }

    /// Returns a mutable reference to the underlying rule.
    ///
    /// # See also
    ///
    /// - [`ToggleableRule::as_rule`]
    #[must_use]
    pub fn as_rule_mut(&mut self) -> &mut R {
        &mut self.rule
    }

    /// Converts this `ToggleableRule<R>` into a type-erased variant.
    ///
    /// This method consumes `self` and returns a `ToggleableRule<Box<dyn ErasedRule>>`,
    /// enabling storage in heterogeneous collections like [`Rules`](crate::rule::Rules).
    ///
    /// # Generic Bounds
    ///
    /// Requires `R: ErasedRule + 'static` to allow boxing and dynamic dispatch.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, ToggleableRule, erased_rule::ErasedRule};
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule = TestRule::new("erase-me", "Erasable rule", Category::Heading, []);
    /// let toggleable = ToggleableRule::new(rule, true);
    ///
    /// let erased = toggleable.into_erased();
    /// assert_eq!(erased.name(), "erase-me");
    /// assert!(erased.is_enabled());
    /// ```
    ///
    /// # See Also
    ///
    /// - [`From` implementation](#impl-From<ToggleableRule<R>>-for-ToggleableRule<Box<(dyn+ErasedRule+'static)>>) — Ergonomic alternative via `.into()`.
    pub fn into_erased(self) -> ToggleableRule<Box<dyn ErasedRule>>
    where
        R: ErasedRule + 'static,
    {
        ToggleableRule::new(Box::new(self.rule), self.enabled)
    }
}

impl<R> From<ToggleableRule<R>> for ToggleableRule<Box<dyn ErasedRule>>
where
    R: ErasedRule + 'static,
{
    fn from(value: ToggleableRule<R>) -> Self {
        value.into_erased()
    }
}

impl<R> Deref for ToggleableRule<R>
where
    R: RuleMetadata,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.rule
    }
}

/// Implements conditional rule execution based on the enabled flag.
///
/// When the rule is **disabled**, `check` returns `Ok(Vec::new())` immediately,
/// without invoking the underlying rule's logic — a zero-cost short-circuit.
///
/// When **enabled**, delegates to `R::check` with the original arguments.
///
impl<R> RuleRunner for ToggleableRule<R>
where
    R: RuleRunner + RuleMetadata,
{
    type Error = R::Error;

    fn check(
        &self,
        content: &super::Content,
        note: &crate::Note,
    ) -> Result<Vec<super::Violation>, Self::Error> {
        if self.is_disabled() {
            return Ok(Vec::new());
        }

        R::check(self, content, note)
    }
}

impl<R> PartialEq for ToggleableRule<R>
where
    R: Rule + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.enabled == other.enabled && self.rule == other.rule
    }
}

impl<L> Eq for ToggleableRule<L> where L: Rule + PartialEq {}

#[cfg(test)]
mod tests {
    use crate::Note;
    use crate::rule::{Category, Content, RuleRunner, ToggleableRule, Violation};
    use crate::test_utils::TestRule;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let rule = TestRule::new("TestRule", "", Category::Content, []);
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
