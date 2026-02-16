use super::Cli;
use crate::command::runner::Runner;
use obsidian_tidy_config::{Config, Error as ConfigError, loader::ConfigLoader};
use obsidian_tidy_core::rule::Content;
use obsidian_tidy_rules::ALL_RULES;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use std::{fs::OpenOptions, path::Path};
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config load error: {0}")]
    Config(#[from] ConfigError),
}

#[derive(Debug, Clone, Default)]
pub struct RunnerCheck;

impl RunnerCheck {
    pub fn new() -> Self {
        Self
    }
}

#[instrument(skip(path))]
fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    ConfigLoader::default()
        .available_rules(&ALL_RULES)
        .load(&mut file)
}

impl Runner for RunnerCheck {
    type Error = self::Error;

    #[instrument]
    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        debug!("Run command `check`");

        let config = load_config(args.config())?;
        let content = Content::new(&args.path);

        let violations: Vec<_> = config
            .rules()
            .par_iter()
            .map(|lint| lint.check(&content).unwrap())
            .flatten()
            .collect();

        for violation in violations {
            println!(
                "{} in `{}`: {}",
                "PROBLEM".red().bold(),
                violation.from().strip_prefix(&args.path).unwrap().display(),
                violation.message().yellow()
            );
        }

        Ok(())
    }
}
