//! Module for load config

use super::Config;
use super::Error;
use obsidian_tidy_core::lint::{DynLint, LintsDeserializer, SerdeLints};
use std::io::Read;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct ConfigLoader<'a> {
    available_lints: &'a Vec<DynLint>,
}

impl<'a> ConfigLoader<'a> {
    pub fn new(available_lints: &'a Vec<DynLint>) -> Self {
        Self { available_lints }
    }

    /// Load config from reader
    #[instrument(skip(reader))]
    pub fn load(self, reader: &mut impl Read) -> Result<Config, Error> {
        debug!("Loading config");

        let seed = LintsDeserializer::new(self.available_lints, &serde_config)
            .deserialise()
            .unwrap();

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let value: toml::Value = toml::from_slice(&buffer)?;
        let config: Li


        Ok(Config { seed })
    }
}
