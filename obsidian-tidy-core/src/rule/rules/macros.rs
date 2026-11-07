/// Convenience macro for constructing a [`Rules`](crate::rule::Rules) collection
/// with **panic-on-duplicate** semantics.
///
/// # Behavior
///
/// - **Empty invocation**: Returns a new empty `Rules` via [`Rules::new`].
/// - **With rules**: Creates a new collection and adds each rule via [`Rules::add`].
///   If `add` returns `Some(prev)` (indicating a duplicate name), the macro panics with:
///   `"Rule with name '{}' already exists"`.
///
/// # Uniqueness Requirement
///
/// Each rule must have a unique name as defined by [`RuleMeta:name`](crate::rule::RuleMetadata::name).
/// This macro enforces uniqueness at runtime via panic. For fallible construction that
/// returns a [`Result`] instead of panicking, use [`try_rules!`].
///
/// # Examples
///
/// ## Creating an empty collection
///
/// ```
/// use obsidian_tidy_core::rules;
///
/// let rules = rules![];
/// assert!(rules.is_empty());
/// ```
///
/// ## Adding rules
///
/// ```
/// use obsidian_tidy_core::rules;
/// use obsidian_tidy_core::rule::{Category, ToggleableRule};
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule_a = TestRule::new("format-headings", "Format headings", Category::Heading, []);
/// let rule_b = TestRule::new("clean-links", "Clean broken links", Category::Content, []);
///
/// let rules = rules![
///     ToggleableRule::new(rule_a, true),
///     ToggleableRule::new(rule_b, false),
/// ];
///
/// assert_eq!(rules.len(), 2);
/// assert!(rules.contains("format-headings"));
/// assert!(rules.contains("clean-links"));
/// ```
///
/// ## Panic on duplicate names
///
/// ```should_panic
/// use obsidian_tidy_core::rules;
/// use obsidian_tidy_core::rule::{Category, ToggleableRule};
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule = TestRule::new("duplicate", "My rule", Category::Content, []);
/// let toggleable = ToggleableRule::new(rule, true);
///
/// // This panics because the same rule name is added twice
/// let _rules = rules![toggleable.clone(), toggleable];
/// ```
///
/// # See Also
///
/// - [`try_rules!`] — Fallible variant that returns `Result<Rules, Value>` instead of panicking.
/// - [`Rules::add`] — The underlying method used to insert rules.
/// - [`Rules::try_from_iter`] — Iterator-based fallible construction.
///
/// [`Rules::new`]: crate::rule::rules::Rules::new
/// [`Rules::add`]: crate::rule::rules::Rules::add
/// [`Rules::try_from_iter`]: crate::rule::rules::Rules::try_from_iter
/// [`try_rules!`]: crate::try_rules
#[macro_export]
macro_rules! rules {
    [] => {
        $crate::rule::rules::Rules::new()
    };

    [$($rule:expr),+ $(,)?] => {{
        let mut rules = $crate::rule::rules::Rules::new();
        $(
            if let Some(prev) = rules.add($rule.into_erased()) {
                panic!("Rule with name '{}' already exists", prev.name());
            }
        )+

        rules
    }};
}

/// Convenience macro for constructing a [`Rules`](crate::rule::Rules) collection
/// with **fallible** semantics.
///
/// # Uniqueness Requirement
///
/// Each rule must have a unique name as defined by [`RuleMetadata::name`](crate::rule::RuleMetadata::name).
/// This macro enforces uniqueness by returning a [`Result`], allowing the caller to handle
/// duplicates gracefully. For eager construction that panics on duplicates, use [`rules!`] instead.
///
/// # Examples
///
/// ## Creating an empty collection
///
/// ```
/// use obsidian_tidy_core::try_rules;
///
/// let rules = try_rules![];
/// assert!(rules.is_ok());
/// assert_eq!(rules.unwrap().len(), 0);
/// ```
///
/// ## Adding rules successfully
///
/// ```
/// use obsidian_tidy_core::rule::{ToggleableRule, Category};
/// use obsidian_tidy_core::test_utils::TestRule;
/// use obsidian_tidy_core::try_rules;
///
/// let rule_a = TestRule::new("format-headings", "Format headings", Category::Heading, []);
/// let rule_b = TestRule::new("clean-links", "Clean broken links", Category::Content, []);
///
/// let result = try_rules![
///     ToggleableRule::new(rule_a, true),
///     ToggleableRule::new(rule_b, false),
/// ];
///
/// let rules = result.expect("Failed to build rules");
/// assert_eq!(rules.len(), 2);
/// assert!(rules.contains("format-headings"));
/// ```
///
/// ## Handling duplicate names gracefully
///
/// ```
/// use obsidian_tidy_core::rule::{ToggleableRule, Category};
/// use obsidian_tidy_core::test_utils::TestRule;
/// use obsidian_tidy_core::try_rules;
///
/// let rule = TestRule::new("duplicate", "My rule", Category::Content, []);
/// let toggleable = ToggleableRule::new(rule, true);
///
/// // Returns Err containing the first rule with the duplicate name
/// let result = try_rules![toggleable.clone(), toggleable];
///
/// assert!(result.is_err());
/// assert_eq!(result.err().unwrap().name(), "duplicate");
/// ```
///
/// # See Also
///
/// - [`rules!`] — Eager variant that panics on duplicate names.
/// - [`Rules::try_from_iter`] — The underlying method used for fallible construction.
/// - [`Rules::add`] — Method for inserting individual rules with duplicate detection.
///
/// [`Rules::new`]: crate::rule::rules::Rules::new
/// [`Rules::add`]: crate::rule::rules::Rules::add
/// [`Rules::try_from_iter`]: crate::rule::rules::Rules::try_from_iter
/// [`rules!`]: crate::rules
#[macro_export]
macro_rules! try_rules {
    [] => {
        $crate::rule::rules::Rules::try_from_iter([])
    };

    [$($rule:expr),+ $(,)?] => {{
        $crate::rule::rules::Rules::try_from_iter([
            $($rule.into_erased()),+
        ])
    }};
}

#[cfg(test)]
mod tests {
    use crate::{
        rule::{RuleMetadata, ToggleableRule},
        test_utils::TestRule,
    };

    #[test]
    fn empty() {
        let rules = rules![];
        assert_eq!(rules.len(), 0);
    }

    #[test]
    fn try_empty() {
        let rules = try_rules![].unwrap();
        assert_eq!(rules.len(), 0);
    }

    #[test]
    fn add() {
        let test_rule = TestRule::default();
        let test_rule = ToggleableRule::new(test_rule, true);
        let rules = rules![test_rule.clone()];

        assert_eq!(rules.len(), 1);
        assert_eq!(rules.names().collect::<Vec<_>>(), [test_rule.name()]);
        assert_eq!(rules.rules().map(|rule| rule.is_enabled()).count(), 1);
    }

    #[test]
    fn try_add() {
        let test_rule = TestRule::default();
        let test_rule = ToggleableRule::new(test_rule, true);
        let rules = try_rules![test_rule.clone()].unwrap();

        assert_eq!(rules.len(), 1);
        assert_eq!(rules.names().collect::<Vec<_>>(), [test_rule.name()]);
        assert_eq!(rules.rules().map(|rule| rule.is_enabled()).count(), 1);
    }

    #[test]
    #[should_panic]
    fn duplicate_add() {
        let test_rule = TestRule::default();
        let test_rule = ToggleableRule::new(test_rule, true);

        let _rules = rules![test_rule.clone(), test_rule];
    }

    #[test]
    fn try_duplicate_add() {
        let test_rule = TestRule::default();
        let test_rule = ToggleableRule::new(test_rule, true);

        let result = try_rules![test_rule.clone(), test_rule];
        assert!(result.is_err());
    }
}
