//! Module for store template rules

use clap::ValueEnum;
use obsidian_tidy_core::rule::{Rules, SharedErrorRule, ToggleableRule};
use obsidian_tidy_rules::ALL_RULES;
use std::{ops::Deref, sync::LazyLock};
use tracing::{instrument, trace};

static ALL: LazyLock<Rules<SharedErrorRule>> = LazyLock::new(|| {
    let rules = ALL_RULES
        .clone()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, true))
        .collect();

    Rules::new(rules).unwrap()
});

static EMPTY: LazyLock<Rules<SharedErrorRule>> = LazyLock::new(|| {
    let rules = ALL_RULES
        .clone()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, false))
        .collect();

    Rules::new(rules).unwrap()
});

static STANDARD: LazyLock<Rules<SharedErrorRule>> = LazyLock::new(|| {
    let mut rules = EMPTY.clone();

    rules["test-rule"].enable();

    rules
});

/// Template config
#[derive(Debug, Clone, ValueEnum)]
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

impl AsRef<Rules<SharedErrorRule>> for Template {
    #[instrument]
    fn as_ref(&self) -> &Rules<SharedErrorRule> {
        trace!("Template to rules");

        match self {
            Template::All => &ALL,
            Template::Standard => &STANDARD,
            Template::Empty => &EMPTY,
        }
    }
}

impl From<Template> for Rules<SharedErrorRule> {
    #[instrument]
    fn from(template: Template) -> Rules<SharedErrorRule> {
        trace!("Template to owned rules");

        match template {
            Template::All => ALL.clone(),
            Template::Standard => STANDARD.clone(),
            Template::Empty => EMPTY.clone(),
        }
    }
}

impl Deref for Template {
    type Target = Rules<SharedErrorRule>;

    fn deref(&self) -> &Self::Target {
        match self {
            Template::All => &ALL,
            Template::Standard => &STANDARD,
            Template::Empty => &EMPTY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Template;
    use obsidian_tidy_rules::ALL_RULES;

    #[test]
    fn all_check() {
        assert_eq!(Template::All.len(), ALL_RULES.len());
        assert!(Template::All.iter().all(|rule| rule.enabled()));
    }

    #[test]
    fn empty_check() {
        assert_eq!(Template::Empty.len(), ALL_RULES.len());
        assert!(Template::Empty.iter().all(|rule| rule.disabled()));
    }

    #[test]
    fn standart() {
        assert_eq!(Template::Standard.len(), ALL_RULES.len());

        assert!(Template::All.iter().any(|rule| rule.enabled()));
        assert!(Template::Empty.iter().any(|rule| rule.disabled()));
    }
}
