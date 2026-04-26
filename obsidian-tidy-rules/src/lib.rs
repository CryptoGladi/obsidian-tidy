//! Module for Rust writer rules

#![forbid(clippy::print_stdout)]

pub mod content;
pub(crate) mod declare_rules;

#[cfg(test)]
pub(crate) mod test_utils;

use declare_rules::declare_rules;

declare_rules!([
    content::empty_content::EmptyContent,
    content::empty_content::fabric()
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_defaults_match() {
        let defaults = create_all_default_rules();

        assert_eq!(defaults.len(), ALL_RULES_FABRICS.len());
        for rule in defaults {
            assert!(ALL_RULES_FABRICS.contains(rule.name()));
        }
    }
}
