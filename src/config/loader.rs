//! Module for load config

use super::Config;
use super::Error;
use obsidian_tidy_core::lint::{DynLint, LintsSeed};
use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer};
use std::io::Read;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    available_lints: &'a Vec<DynLint>,
}

#[derive(Debug)]
struct ConfigSeed<'a> {
    lint_seed: &'a LintsSeed<'a>,
}

impl<'a> ConfigSeed<'a> {
    fn new(lint_seed: &'a LintsSeed<'a>) -> Self {
        Self { lint_seed }
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
            lints: toml::Value,
        }

        let inner = InnerConfig::deserialize(deserializer)?;

        Ok(Self::Value {
            lints: self.lint_seed.deserialize(inner.lints).unwrap(),
        })
    }
}

impl<'a> ConfigLoader<'a> {
    pub fn new(available_lints: &'a Vec<DynLint>) -> Self {
        Self { available_lints }
    }

    /// Load config from reader
    #[instrument(skip(reader), err)]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, Error> {
        debug!("Loading config");

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let lint_seed = LintsSeed::new(self.available_lints);

        Ok(ConfigSeed::new(&lint_seed).deserialize(toml::Deserializer::parse(&buffer)?)?)
    }
}
