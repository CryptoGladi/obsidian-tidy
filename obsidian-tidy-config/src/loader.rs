//! Module for load config

use super::Config;
use super::Error;
use obsidian_tidy_core::rule::{RulesSeed, SharedErrorRule};
use obsidian_tidy_rules::ALL_RULES;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer};
use std::io::Read;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    available_rules: &'a Vec<SharedErrorRule>,
}

impl Default for ConfigLoader<'_> {
    fn default() -> Self {
        Self::new(&ALL_RULES)
    }
}

#[derive(Debug)]
struct ConfigSeed<'a> {
    rule_seed: &'a RulesSeed<'a, SharedErrorRule>,
}

impl<'a> ConfigSeed<'a> {
    const fn new(rule_seed: &'a RulesSeed<'a, SharedErrorRule>) -> Self {
        Self { rule_seed }
    }
}

impl<'de> DeserializeSeed<'de> for ConfigSeed<'_> {
    type Value = Config;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerConfig {
            rules: toml::Value,
        }

        let inner = InnerConfig::deserialize(deserializer)?;

        Ok(Self::Value {
            rules: self.rule_seed.clone().deserialize(inner.rules).unwrap(),
        })
    }
}

impl<'a> ConfigLoader<'a> {
    #[must_use]
    pub const fn new(available_rules: &'a Vec<SharedErrorRule>) -> Self {
        Self { available_rules }
    }

    #[must_use]
    pub const fn available_rules(mut self, available_rules: &'a Vec<SharedErrorRule>) -> Self {
        self.available_rules = available_rules;
        self
    }

    /// Load config from reader
    #[instrument(skip(reader), err)]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, Error> {
        debug!("Loading config");

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let rule_seed = RulesSeed::new(self.available_rules);
        let toml = toml::Deserializer::parse(&buffer)?;

        let config = ConfigSeed::new(&rule_seed).deserialize(toml)?;
        Ok(config)
    }
}
