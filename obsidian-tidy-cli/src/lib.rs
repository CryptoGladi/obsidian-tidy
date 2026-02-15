//! Module for CLI interfaic

pub mod command;

use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;

pub use clap::Parser;
pub use command::Command;

/// Returns the current working directory
fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or(PathBuf::from("."))
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
    #[arg(long, value_name = "DIRECTORY", default_value = current_dir().into_os_string())]
    pub path: PathBuf,

    /// Nothing is output to stdout.
    #[arg(long)]
    pub quiet: bool,

    /// Path to directory for logs
    /// Default save to locale share data
    #[arg(long, value_name = "DIRECTORY", default_value = directories().logs_dir().into_os_string())]
    pub logs: PathBuf,

    /// Command
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    /// Return path to config
    pub fn config(&self) -> PathBuf {
        self.path.join(".obsidian-tidy.toml")
    }
}
