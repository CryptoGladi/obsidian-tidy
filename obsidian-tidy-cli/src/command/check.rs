//! Run check to vault

use super::Cli;
use crate::command::runner::Runner;
use obsidian_parser::prelude::Note as _;
use obsidian_tidy_config::{Config, Error as ConfigError, loader::ConfigLoader};
use obsidian_tidy_core::rule::{Content, Violation};
use obsidian_tidy_rules::ALL_RULES;
use rayon::prelude::*;
use std::{
    fs::OpenOptions,
    ops::Range,
    path::{Path, PathBuf},
};
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

#[derive(Debug)]
#[allow(unused)]
pub struct Diagnostic {
    message: String,
    location: Range<usize>,
    path: PathBuf,
}

impl Diagnostic {
    fn from_violation(violation: &Violation, path: impl Into<PathBuf>) -> Self {
        Self {
            message: violation.message().to_string(),
            location: violation.location().clone(),
            path: path.into(),
        }
    }
}

impl Runner for RunnerCheck {
    type Error = self::Error;

    #[instrument]
    #[allow(unused)]
    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        debug!("Run command `check`");

        let config = load_config(args.config())?;
        let content = Content::new(&args.path);

        let notes_to_check = content.vault.notes();
        let diagnostics: Vec<Diagnostic> = notes_to_check
            .par_iter()
            .flat_map(|note| {
                config
                    .rules()
                    .iter()
                    .filter_map(|rule| match rule.check(&content, note) {
                        Ok(violations) => Some(violations),
                        Err(e) => {
                            eprintln!("Rule '{}' failed: {}", rule.name(), e);
                            None
                        }
                    })
                    .flatten()
                    .map(|violation| Diagnostic::from_violation(&violation, note.path().unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect();

        println!("{:?}", diagnostics);

        /*
        for violation in violations {
            println!(
                "{} in `{}`: {}",
                "PROBLEM".red().bold(),
                violation.from().strip_prefix(&args.path).unwrap().display(),
                violation.message().yellow()
            );
        }
        */

        Ok(())
    }
}
