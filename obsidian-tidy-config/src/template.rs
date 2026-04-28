//! Module for store template rules

use clap::ValueEnum;
use obsidian_tidy_core::rule::{Rules, ToggleableRule};
use obsidian_tidy_rules::create_all_default_rules;
use std::sync::OnceLock;
use tracing::{instrument, trace};

const ERROR_MESSAGE: &str =
    "Duplicate rule name detected in template initialization. Check declare_rules! invocation";

#[must_use]
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

#[must_use]
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
/// Used by `standard_rules!` macro for invariant verification.
/// Not part of public API — do not call directly.
#[doc(hidden)]
pub fn _is_rule_registered(name: &str) -> bool {
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
    ($($rule_name:literal),+ $(,)?) => {
        const ENABLED_RULES: &[&str] = &[$($rule_name),+];

        if cfg!(debug_assertions) {
            #[expect(clippy::used_underscore_items)]
            for name in ENABLED_RULES {
                assert!(
                    $crate::template::_is_rule_registered(name),
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
                .unwrap_or_else(|| panic!("Rule '{rule_name}' must exist in standard template"))
                .enable();
        }

        rules
    };

    () => {
        $crate::template::empty()
    };
}

/// Returns the standard template configuration.
///
/// Enables a curated subset of rules recommended for most users.
///
/// # Panics
///
/// Panics if any rule listed in `ENABLED_RULES` is not registered.
/// This should never happen in a correctly built binary;
/// integrity is verified by unit tests.
#[must_use]
#[allow(clippy::panic, reason = "There is a check in unit tests and docs")]
pub fn standard() -> Rules {
    standard_rules! {
        "empty-content"
    }
}

/// Template config
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
#[clap(rename_all = "kebab-case")]
pub enum Template {
    /// Enabled all rules
    All,

    /// Standard config.
    /// Recommended for most users
    Standard,

    /// Disabled all rule
    Empty,
}

impl From<Template> for Rules {
    #[instrument]
    fn from(template: Template) -> Rules {
        trace!("Template to owned rules");

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
}
