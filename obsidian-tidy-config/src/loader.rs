//! Module for load config

use super::Config;
use obsidian_tidy_core::rule::RuleFabricRegistry;
use obsidian_tidy_core::rule::RulesSeed;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::io::Read;
use strum::{IntoStaticStr, VariantNames};
use tracing::instrument;

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    available_rules: &'a RuleFabricRegistry,
}

impl<'a> ConfigLoader<'a> {
    #[must_use]
    pub const fn new(available_rules: &'a RuleFabricRegistry) -> Self {
        Self { available_rules }
    }

    /// Load config from reader
    #[instrument(skip(reader), err)]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, crate::Error> {
        tracing::debug!("loading config");

        let mut json = serde_json::Deserializer::from_reader(reader);

        let rule_seed = RulesSeed::new(self.available_rules);
        let config = ConfigSeed::new(rule_seed).deserialize(&mut json)?;

        Ok(config)
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

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                tracing::trace!("run visit_map");
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
                Ok(Config { rules })
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
    use crate::ConfigSaver;
    use obsidian_tidy_core::{
        rule::{GetErasedRule, Rules, ToggleableRule},
        rule_fabric_registry,
        test_utils::{TestRule, TestRuleFabric},
    };
    use std::io::Cursor;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn load() {
        let mut rules = Rules::new();
        if let Some(prev) = rules.add(ToggleableRule::new(TestRule::default().into_erased(), true))
        {
            panic!("found duplicate: {:?}", prev);
        }

        let config = Config { rules };

        let mut buffer = Vec::new();
        ConfigSaver::new(&config).save(&mut buffer).unwrap();

        tracing::debug!("DATA: {}", String::from_utf8(buffer.clone()).unwrap());
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = ConfigLoader::new(&registry);
        loader.load(&mut Cursor::new(buffer)).unwrap();
    }
}
