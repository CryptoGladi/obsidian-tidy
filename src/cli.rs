//! Module for CLI interfaic

use crate::config::template::Template;
use clap::{Parser, Subcommand};
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;

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
pub struct CLI {
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

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run lints
    Check,

    /// Initialization of config for linter
    Init {
        /// Override config
        #[arg(long = "override")]
        override_config: bool,

        /// How template use?
        #[arg(long, value_enum, default_value_t = Template::Standard)]
        template: Template,
    },
}
