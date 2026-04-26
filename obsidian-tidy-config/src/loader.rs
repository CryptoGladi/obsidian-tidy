//! Module for load config

use super::Config;
use obsidian_tidy_core::rule::RuleFabricRegistry;
use obsidian_tidy_core::rule::RulesSeed;
use serde::Deserializer;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use std::io::Read;
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

    #[must_use]
    pub const fn available_rules(mut self, available_rules: &'a RuleFabricRegistry) -> Self {
        self.available_rules = available_rules;
        self
    }

    /// Load config from reader
    #[instrument(skip(reader), err)]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, crate::Error> {
        tracing::debug!("Loading config");

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

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

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        const RULES_FIELD: &str = "rules";
        const FIELDS: &[&str] = &[RULES_FIELD];
        struct ConfigVisitor<'a> {
            rule_seed: RulesSeed<'a>,
        }

        impl<'de, 'a> Visitor<'de> for ConfigVisitor<'a> {
            type Value = Config;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct Config with fields: {FIELDS:?}")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut rules = None;

                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        RULES_FIELD => {
                            if rules.is_some() {
                                return Err(M::Error::duplicate_field(RULES_FIELD));
                            }

                            rules = Some(map.next_value_seed(self.rule_seed.clone())?);
                        }
                        other => {
                            return Err(M::Error::unknown_field(other, FIELDS));
                        }
                    }
                }

                let rules = rules.ok_or_else(|| M::Error::missing_field(RULES_FIELD))?;
                Ok(Config { rules })
            }
        }

        deserializer.deserialize_struct(
            "Config",
            &FIELDS,
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
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::{NamedTempFile, tempfile};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn load() {
        let mut rules = Rules::new();
        if let Some(prev) = rules.add(ToggleableRule::new(TestRule::default().into_erased(), true))
        {
            panic!("Found duplicate: {:?}", prev);
        }

        let config = Config { rules };

        let mut tempfile = tempfile().unwrap();
        ConfigSaver::new(&config).save(&mut tempfile).unwrap();

        //println!("Path: {}", tempfile.path().display());
        tempfile.flush().unwrap();
        tempfile.seek(SeekFrom::Start(0)).unwrap();

        //println!("Path: {}", tempfile.path().display());
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let loader = ConfigLoader::new(&registry);
        //println!("Path: {}", tempfile.path().display());
        loader.load(&mut tempfile).unwrap();
    }
}
