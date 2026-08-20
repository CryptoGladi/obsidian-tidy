//! Core configuration type and construction logic.
//!
//! This module defines the [`Config`] struct, which holds the active set of
//! tidying rules. Construction is restricted to prevent invalid states:
//!
//! The [`private::Token`] pattern ensures that external crates cannot bypass
//! these construction paths.

mod private {
    /// Zero-sized token type that grants permission to construct [`Config`].
    ///
    /// This type is private to the [`config`] module, so only submodules
    /// ([`loader`], [`saver`]) can create instances of it.
    #[derive(Debug, Clone, Copy)]
    pub(super) struct Token;
}

use crate::{loader::ConfigLoader, saver::ConfigSaver, template::Template};
use obsidian_tidy_core::rule::{RuleFactoryRegistry, Rules};
use serde::{Serialize, Deserialize};
use tracing::instrument;

/// The main configuration struct for obsidian-tidy.
///
/// # Construction
///
/// ```
/// # use obsidian_tidy_config::{Config, Template};
/// // Recommended: create from a template
/// let config = Config::new(Template::Standard);
///
/// // Or use the `From` impl
/// let config: Config = Template::Empty.into();
/// ```
///
/// # Serialization
///
/// ```
/// # use obsidian_tidy_config::{Config, Template};
/// let config = Config::new(Template::All);
///
/// let mut output = Vec::new();
/// config.saver().save(&mut output)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Deserialization
///
/// Requires a [`RuleFabricRegistry`] to validate rule names during loading:
///
/// ```no_run
/// # use obsidian_tidy_config::Config;
/// # use obsidian_tidy_core::rule_fabric_registry;
/// # use std::io::Cursor;
/// use obsidian_tidy_rules::ALL_RULES_FABRICS;
///
/// // let registry = rule_fabric_registry![/* your factories */];
/// let config = Config::loader(&ALL_RULES_FABRICS)
///     .load(Cursor::new(b"{\"rules\":{}}"))?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Config {
    pub rules: Rules,

    #[serde(skip)]
    _private: private::Token, // This prevents external construction
}

impl Config {
    /// Creates a new `Config` from a [`Template`].
    ///
    /// This is the primary public constructor. It converts the template into
    /// a concrete set of rules, applying default parameters for each enabled
    /// rule.
    ///
    /// # Example
    ///
    /// ```
    /// # use obsidian_tidy_config::{Config, Template};
    /// let config = Config::new(Template::Standard);
    /// assert!(!config.rules.is_empty());
    /// ```
    #[instrument(level = "trace")]
    pub fn new(template: Template) -> Self {
        Self {
            rules: template.into(),
            _private: private::Token,
        }
    }

    /// Returns a [`ConfigLoader`] for deserializing configuration from input.
    #[must_use = "ConfigLoader does nothing unless you call .load()"]
    pub const fn loader(registry: &RuleFactoryRegistry) -> ConfigLoader<'_> {
        ConfigLoader { registry }
    }

    /// Returns a [`ConfigSaver`] for serializing this configuration to output.
    #[must_use = "ConfigSaver does nothing unless you call .save()"]
    pub const fn saver(&self) -> ConfigSaver<'_> {
        ConfigSaver::new(self)
    }

    /// Internal constructor for bypassing normal construction.
    ///
    /// # Usage
    ///
    /// | Caller | Context | Responsibility |
    /// |--------|---------|---------------|
    /// | [`ConfigLoader`] | Deserialization | Ensure deserialized `rules` are valid |
    /// | Unit tests | `#[cfg(test)]` | Set up arbitrary rules for test scenarios |
    #[doc(hidden)]
    pub(crate) const fn __from_raw(rules: Rules) -> Self {
        Self {
            rules,
            _private: private::Token,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(Template::default())
    }
}

impl From<Template> for Config {
    fn from(template: Template) -> Self {
        Self::new(template)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obsidian_tidy_core::rule::ToggleableRule;

    #[test]
    fn new() {
        let config = Config::new(Template::Empty);

        assert_eq!(
            config.rules.len(),
            obsidian_tidy_rules::ALL_RULES_FABRICS.len()
        );

        assert!(config.rules.rules().all(ToggleableRule::is_disabled));
    }

    #[test]
    fn from_template() {
        let template = Template::All;
        let config: Config = template.into();

        assert!(config.rules.rules().all(ToggleableRule::is_enabled));
    }
}
