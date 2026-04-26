//! Module for store template rules

use clap::ValueEnum;
use obsidian_tidy_core::rule::{Rules, ToggleableRule};
use obsidian_tidy_rules::create_all_default_rules;
use tracing::{instrument, trace};

const ERROR_MESSAGE: &str =
    "Duplicate rule name detected in template initialization. Check declare_rules! invocation";

pub fn all() -> Rules {
    let rules = create_all_default_rules()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, true));

    Rules::try_from_iter(rules).expect(ERROR_MESSAGE)
}

pub fn empty() -> Rules {
    let rules = create_all_default_rules()
        .into_iter()
        .map(|rule| ToggleableRule::new(rule, false));

    Rules::try_from_iter(rules).expect(ERROR_MESSAGE)
}

pub fn standard() -> Rules {
    let mut rules = self::empty();

    rules["empty-content"].enable();

    rules
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
