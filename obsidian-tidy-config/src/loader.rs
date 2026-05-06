//! Deserialization logic for loading configuration from input.

use super::{Config, Error};
use obsidian_tidy_core::rule::RuleFabricRegistry;
use obsidian_tidy_core::rule::RulesSeed;
use serde::de::{DeserializeSeed, Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::io::Read;
use strum::{IntoStaticStr, VariantNames};
use tracing::instrument;

/// A loader for deserializing [`Config`] from input.
///
/// Created via [`Config::loader()`].
///
/// # Example
///
/// ```
/// use obsidian_tidy_config::Config;
/// use obsidian_tidy_core::rule_fabric_registry;
/// use obsidian_tidy_rules::ALL_RULES_FABRICS;
/// use std::io::Cursor;
///
/// // let registry = rule_fabric_registry![/* ... */];
///
/// let loader = Config::loader(&ALL_RULES_FABRICS);
/// let config = loader.load(Cursor::new(b"{\"rules\":{}}"))?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct ConfigLoader<'a> {
    pub(crate) registry: &'a RuleFabricRegistry,
}

impl ConfigLoader<'_> {
    /// Loads a [`Config`] from the given reader.
    #[instrument(skip(reader), level = "debug", err)]
    pub fn load(self, reader: impl Read) -> Result<Config, Error> {
        let mut json = serde_json::Deserializer::from_reader(reader);

        let rule_seed = RulesSeed::new(self.registry);
        ConfigSeed::new(rule_seed)
            .deserialize(&mut json)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
struct ConfigSeed<'a> {
    rule_seed: RulesSeed<'a>,
}

impl<'a> ConfigSeed<'a> {
    const fn new(rule_seed: RulesSeed<'a>) -> Self {
        Self { rule_seed }
    }
}

impl<'de> DeserializeSeed<'de> for ConfigSeed<'de> {
    type Value = Config;

    #[instrument(skip_all, err)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, IntoStaticStr, VariantNames, strum::Display)]
        #[serde(field_identifier, rename_all = "lowercase")]
        #[strum(serialize_all = "lowercase")]
        enum Field {
            Rules,
        }

        struct ConfigVisitor<'a> {
            rule_seed: RulesSeed<'a>,
        }

        impl<'de> Visitor<'de> for ConfigVisitor<'_> {
            type Value = Config;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "struct Config with fields: `{}`",
                    Field::VARIANTS.join("`, `")
                )
            }

            #[instrument(skip_all, level = "trace")]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut rules = None;

                while let Some(key) = map.next_key::<Field>()? {
                    tracing::trace!("found key: `{key}`");

                    match key {
                        Field::Rules => {
                            if rules.is_some() {
                                return Err(M::Error::duplicate_field(Field::Rules.into()));
                            }

                            rules = Some(map.next_value_seed(self.rule_seed)?);
                        }
                    }
                }

                let rules = rules.ok_or_else(|| M::Error::missing_field(Field::Rules.into()))?;
                Ok(Config::__from_raw(rules))
            }
        }

        deserializer.deserialize_struct(
            "Config",
            Field::VARIANTS,
            ConfigVisitor {
                rule_seed: self.rule_seed,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obsidian_tidy_core::{
        rule::ToggleableRule,
        rule_fabric_registry, rules,
        test_utils::{TestRule, TestRuleFabric},
    };
    use std::io::Cursor;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn save_and_load() {
        let test_rule = {
            let test_rule = TestRule::default();
            ToggleableRule::new(test_rule, true)
        };

        let rules = rules![test_rule];
        let config = Config::__from_raw(rules);

        let mut buffer = Vec::new();
        config.saver().save(&mut buffer).unwrap();
        tracing::debug!("DATA: {}", String::from_utf8(buffer.clone()).unwrap());

        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loaded = Config::loader(&registry).load(Cursor::new(buffer)).unwrap();
        assert!(loaded.rules.contains("test-rule"));
    }

    #[test]
    #[traced_test]
    fn empty_load() {
        let json = r#"{"rules": {}}"#;
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = Config::loader(&registry);
        let config = loader.load(Cursor::new(json)).unwrap();

        assert_eq!(config.rules.len(), 0); // TestRule зарегистрирован, но выключен
    }

    #[test]
    #[traced_test]
    fn missing_rules_field() {
        let json = r#"{}"#;
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = Config::loader(&registry);
        let error = loader.load(Cursor::new(json)).unwrap_err();

        assert!(error.to_string().contains("missing field `rules`"));
    }

    #[test]
    #[traced_test]
    fn duplicate_rules_field() {
        let json = r#"{"rules": {}, "rules": {}}"#;
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = Config::loader(&registry);
        let error = loader.load(Cursor::new(json)).unwrap_err();

        assert!(error.to_string().contains("duplicate field `rules`"));
    }

    #[test]
    #[traced_test]
    fn unknown_field() {
        let json = r#"{"rules": {}, "oh-my": "123"}"#;
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = Config::loader(&registry);
        let error = loader.load(Cursor::new(json)).unwrap_err();

        assert!(error.to_string().contains("unknown field `oh-my`"));
    }

    #[test]
    #[traced_test]
    fn case_sensitive_field_names() {
        let json = r#"{"Rules": {}}"#; // Capital R
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = Config::loader(&registry);
        let error = loader.load(Cursor::new(json)).unwrap_err();
        let error_msg = error.to_string();

        let messages = ["missing field `rules`", "unknown field `Rules`"];

        // serde may return one of two errors depending on the version/context
        let has_error = messages.iter().any(|expected| error_msg.contains(expected));

        assert!(
            has_error,
            "Expected error to contain one of [{messages:?}], got: `{error}`",
        );
    }
}
