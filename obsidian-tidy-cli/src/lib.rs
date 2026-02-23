//! Module for CLI interfaic

pub mod command;
pub mod logger_config;

use clap::ValueHint;
use std::path::PathBuf;

pub use clap::Parser;
pub use command::Command;
pub use logger_config::{LogLevel, LoggerConfig};

/// Returns the current working directory
fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or(PathBuf::from("."))
}

fn existing_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if path.exists() && path.is_dir() {
        Ok(path)
    } else {
        Err(format!("'{s}' is not an existing directory"))
    }
}

/// CLI
#[derive(Debug, Parser)]
#[command(name = "obsidian-tidy")]
#[command(
    version,
    about = "🚀 Blazingly fast Obsidian vault linter",
    long_about = None
)]
pub struct Cli {
    /// Path to Obsidian vault
    #[arg(long, value_name = "DIRECTORY", value_hint = ValueHint::DirPath, value_parser = existing_dir, default_value = current_dir().into_os_string())]
    pub path: PathBuf,

    /// Logger options
    #[command(flatten, next_help_heading = "Logger options")]
    pub logger: LoggerConfig,

    /// Command
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    /// Return path to config
    #[must_use]
    pub fn config(&self) -> PathBuf {
        self.path.join(".obsidian-tidy.toml")
    }
}
