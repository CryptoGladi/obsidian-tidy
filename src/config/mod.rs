//! Module for configuration

pub mod builder;
pub mod error;
pub mod loader;
pub mod saver;
pub mod template;

use obsidian_tidy_core::lint::{Lints, WrappedAnyhowError};
use serde::Serialize;
use std::{fs::OpenOptions, path::Path};
use thiserror::Error;
use tracing::{debug, instrument};

pub use error::Error;
pub use loader::ConfigLoader;
pub use saver::ConfigSaver;
pub use template::Template;

use crate::config::builder::ConfigBuilder;

#[derive(Debug, Serialize)]
pub struct Config {
    lints: Lints<WrappedAnyhowError>,
}

#[instrument(skip(path))]
pub fn init_command(
    path: impl AsRef<Path>,
    override_config: bool,
    template: Template,
) -> anyhow::Result<()> {
    debug!("Init config");
    let path = path.as_ref();

    if path.is_file() {
        match override_config {
            true => std::fs::remove_file(path)?,
            false => anyhow::bail!("Config file already exists. Use `--override`"),
        };
    }

    let config = ConfigBuilder::default().lints(template.into()).build();

    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    ConfigSaver::new(&config).path(path).save(&mut file)?;

    Ok(())
}
