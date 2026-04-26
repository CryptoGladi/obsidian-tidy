//! Module for save Config

use super::{Config, Error};
use std::io::Write;
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct ConfigSaver<'a> {
    config: &'a Config,
}

impl<'a> ConfigSaver<'a> {
    #[must_use]
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// Save config to writer
    #[instrument(skip(writer), err)]
    pub fn save(&self, writer: &mut impl Write) -> Result<(), Error> {
        debug!("Save config");

        let json = serde_json::to_string_pretty(self.config)?;
        writer.write_all(json.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}
