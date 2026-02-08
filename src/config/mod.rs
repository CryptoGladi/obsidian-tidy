use obsidian_tidy_core::lint::Lints;
use obsidian_tidy_lints::template;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Toml deserialize error `{0}`")]
    Deserialize(#[from] toml::de::Error),

    #[error("Toml serialize error `{0}`")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Serialize)]
pub struct Config {
    lints: Lints,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lints: template::empty(),
        }
    }
}

impl Config {
    #[instrument(skip_all)]
    pub fn load(reader: &mut impl Read) -> Result<Self, Error> {
        debug!("Loading config");

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        //Ok(toml::from_slice(&buffer)?)
        todo!()
    }

    #[instrument(skip_all)]
    pub fn save(&self, writer: &mut impl Write) -> Result<(), Error> {
        debug!("Save config");

        let toml = toml::to_string_pretty(&self)?;
        writer.write_all(toml.as_bytes())?;
        Ok(())
    }
}
