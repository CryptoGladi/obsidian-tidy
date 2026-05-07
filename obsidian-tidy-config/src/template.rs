//! Predefined configuration templates.
//!
//! Templates provide convenient starting points for configuration:
//! - [`Template::All`]: Enable every registered rule
//! - [`Template::Standard`]: Curated subset recommended for most users
//! - [`Template::Empty`]: Disable all rules (for custom setup)
//!
//! This module also provides the [`standard_rules!`] macro for defining
//! custom template presets with compile-time validation.
//!
//! [`standard_rules!`]: crate::standard_rules

use clap::ValueEnum;
use obsidian_tidy_core::rule::{Rules, ToggleableRule};
use obsidian_tidy_rules::create_all_default_rules;
use std::sync::OnceLock;
use tracing::{instrument, trace};

const ERROR_MESSAGE: &str =
    "Duplicate rule name detected in template initialization. Check declare_rules! invocation";

/// Returns a [`Rules`] collection with all rules enabled.
///
/// # Panics
///
/// Panics if [`create_all_default_rules()`] returns duplicate rule names.
/// This indicates a programming error in rule registration; run
/// `cargo test` to validate integrity before release.
///
/// # Performance
///
/// This function is not cached. For repeated calls, consider storing the
/// result in a `OnceLock<Rules>` at the call site.
#[must_use]
#[track_caller]
#[allow(
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "There is a check in unit tests"
)]
pub fn all() -> Rules {
    let rules = create_all_default_rules()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, true));

    Rules::try_from_iter(rules).expect(ERROR_MESSAGE)
}

/// Returns a [`Rules`] collection with all rules disabled.
///
/// # Panics
///
/// Panics if [`create_all_default_rules()`] returns duplicate rule names.
/// See [`all()`] for details.
#[must_use]
#[track_caller]
#[allow(
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "There is a check in unit tests"
)]
pub fn empty() -> Rules {
    let rules = create_all_default_rules()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, false));

    Rules::try_from_iter(rules).expect(ERROR_MESSAGE)
}

/// Internal helper: checks if a rule with given name is registered.
///
/// Used by [`standard_rules!`] macro for invariant verification.
/// Not part of public API — do not call directly.
#[doc(hidden)]
pub fn __is_rule_registered(name: &str) -> bool {
    static REGISTRY: OnceLock<Rules> = OnceLock::new();
    let registry = REGISTRY.get_or_init(self::empty);

    registry.contains(name)
}

/// Macro to define the standard template rules with automatic test generation.
///
/// # Usage
/// ```
/// use obsidian_tidy_core::rule::Rules;
/// use obsidian_tidy_config::standard_rules;
///
/// pub fn standard() -> Rules {
///     standard_rules! {
///         "empty-content",
///         "trailing-spaces"
///     }
/// }
/// ```
///
/// # What it generates
/// - `const ENABLED_RULES`: slice with rule names for iteration
/// - Runtime loop: enables each listed rule in the `rules` variable
#[macro_export]
macro_rules! standard_rules {
    ($($rule_name:literal),+ $(,)?) => {{
        const ENABLED_RULES: &[&str] = &[$($rule_name),+];

        if cfg!(debug_assertions) {
            for name in ENABLED_RULES {
                assert!(
                    $crate::template::__is_rule_registered(name),
                    "Standard template invariant broken: rule '{}' is not registered. \
                     Run `cargo test` to catch this at compile-time.",
                    name
                );
            }
        }

        let mut rules = $crate::template::empty();
        for rule_name in ENABLED_RULES {
            rules
                .get_mut(rule_name)
                .unwrap_or_else(|| panic!("Rule '{}' must exist in standard template", rule_name))
                .enable();
        }

        rules
    }};

    () => {{
        $crate::template::empty()
    }};
}

/// Returns the standard template configuration.
///
/// Enables a curated subset of rules recommended for most users.
///
/// # Panics
///
/// Panics if any rule listed in the macro is not registered. This should
/// never happen in a correctly built binary; integrity is verified by unit
/// tests.
///
/// # Extending
///
/// To customize the standard preset, define your own function using
/// [`standard_rules!`] instead of calling this one.
///
/// [`standard_rules!`]: crate::standard_rules
#[must_use]
#[track_caller]
#[allow(clippy::panic, reason = "There is a check in unit tests and docs")]
pub fn standard() -> Rules {
    standard_rules! {
        "empty-content"
    }
}

/// Template config
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, strum::Display)]
#[clap(rename_all = "kebab-case")]
pub enum Template {
    /// Enabled all rules
    All,

    /// Standard configuration: curated subset recommended for most users.
    #[default]
    Standard,

    /// Disable all rules. Start from a clean slate.
    Empty,
}

impl From<Template> for Rules {
    #[instrument]
    fn from(template: Template) -> Rules {
        trace!("template to owned rules");

        match template {
            Template::All => all(),
            Template::Standard => standard(),
            Template::Empty => empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obsidian_tidy_rules::ALL_RULES_FABRICS;

    #[test]
    fn all_check() {
        let all = Rules::from(Template::All);

        assert_eq!(all.len(), ALL_RULES_FABRICS.len());
        assert!(all.rules().all(ToggleableRule::is_enabled));
    }

    #[test]
    fn empty_check() {
        let empty = Rules::from(Template::Empty);

        assert_eq!(empty.len(), ALL_RULES_FABRICS.len());
        assert!(empty.rules().all(ToggleableRule::is_disabled));
    }

    #[test]
    fn standard_check() {
        let standard = Rules::from(Template::Standard);

        assert_eq!(standard.len(), ALL_RULES_FABRICS.len());
        //assert!(standard.rules().any(ToggleableRule::is_enabled));
        //assert!(standard.rules().any(ToggleableRule::is_disabled));
        // TODO
    }

    #[test]
    fn template_default_is_standard() {
        assert_eq!(Template::default(), Template::Standard);
    }

    #[test]
    fn standard_rules_macro_empty() {
        let rules = standard_rules![];

        assert_eq!(rules.len(), ALL_RULES_FABRICS.len());
        assert!(rules.rules().all(ToggleableRule::is_disabled));
    }

    #[test]
    fn standard_rules_macro_single_rule() {
        let rules = standard_rules!["empty-content"];

        assert!(rules.contains("empty-content"));
        assert!(rules.get("empty-content").unwrap().is_enabled());

        for rule in rules.rules() {
            if rule.name() != "empty-content" {
                assert!(rule.is_disabled());
            }
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Standard template invariant broken")]
    fn standard_rules_macro_unknown_rule_debug() {
        let _rules = standard_rules!["this-rule-does-not-exist-ever"];
    }

    #[test]
    fn is_rule_registered_known_rule() {
        assert!(__is_rule_registered("empty-content"));
    }

    #[test]
    fn is_rule_registered_unknown_rule() {
        assert!(!__is_rule_registered("fake-rule-12345"));
    }

    #[test]
    fn is_rule_registered_once_lock_idempotent() {
        let r1 = __is_rule_registered("empty-content");
        let r2 = __is_rule_registered("empty-content");

        assert_eq!(r1, r2);
    }
}
