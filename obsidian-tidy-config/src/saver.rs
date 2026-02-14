//! Module for save Config

use super::{Config, Error};
use std::{io::Write, path::PathBuf};
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct ConfigSaver<'a> {
    path: PathBuf,
    config: &'a Config,
}

impl<'a> ConfigSaver<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            path: std::env::current_dir().unwrap_or(PathBuf::from(".")),
            config,
        }
    }

    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    /// Save config to writer
    #[instrument(skip(writer), err)]
    pub fn save(&self, writer: &mut impl Write) -> Result<(), Error> {
        debug!("Save config");

        let toml = toml::to_string(self.config)?;
        writer.write_all(toml.as_bytes())?;

        Ok(())
    }
}
