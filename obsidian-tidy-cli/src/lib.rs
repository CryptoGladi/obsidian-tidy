//! Module for CLI interface

pub mod command;
pub mod config;
pub mod logger_config;

use clap::ValueHint;
use std::path::PathBuf;

pub use clap::Parser;
pub use command::Command;
pub use logger_config::{LogLevel, LoggerConfig};

/// Returns the current working directory
fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("Warning: failed to get current directory: {e}");

        PathBuf::from(".")
    })
}

fn existing_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if !path.exists() {
        Err(format!("Directory '{}' does not exist", s))
    } else if !path.is_dir() {
        Err(format!("'{}' is not a directory", path.display()))
    } else {
        Ok(path)
    }
}

/// CLI
#[derive(Debug, Parser)]
#[command(
    name = "obsidian-tidy",
    version,
    about = "Blazingly fast Obsidian vault linter",
    long_about = None,
    arg_required_else_help = true,
    next_help_heading = "Global options"
)]
pub struct Cli {
    /// Path to Obsidian vault root
    #[arg(
        long,
        short = 'p',
        value_name = "DIR",
        value_hint = ValueHint::DirPath,
        value_parser = existing_dir,
        default_value = current_dir().into_os_string(),
        help_heading = "Input"
    )]
    pub path: PathBuf,

    /// Logger configuration
    #[command(flatten, next_help_heading = "Output")]
    pub logger: LoggerConfig,

    /// Subcommand to execute
    #[command(subcommand, next_help_heading = "Commands")]
    pub command: Command,
}
