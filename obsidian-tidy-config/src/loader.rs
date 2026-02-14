//! Module for load config

use super::Config;
use super::Error;
use obsidian_tidy_core::rule::{RulesSeed, SharedErrorRule};
use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer};
use std::io::Read;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    available_rules: &'a Vec<SharedErrorRule>,
}

#[derive(Debug)]
struct ConfigSeed<'a> {
    rule_seed: &'a RulesSeed<'a, SharedErrorRule>,
}

impl<'a> ConfigSeed<'a> {
    fn new(rule_seed: &'a RulesSeed<'a, SharedErrorRule>) -> Self {
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
    pub fn new(available_rules: &'a Vec<SharedErrorRule>) -> Self {
        Self { available_rules }
    }

    /// Load config from reader
    #[instrument(skip(reader), err)]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, Error> {
        debug!("Loading config");

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let rule_seed = RulesSeed::new(self.available_rules);

        Ok(ConfigSeed::new(&rule_seed).deserialize(toml::Deserializer::parse(&buffer)?)?)
    }
}
