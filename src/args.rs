use clap::{Parser, Subcommand, ValueEnum};
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;

fn default_path() -> PathBuf {
    std::env::current_dir().unwrap_or(PathBuf::from("."))
}

#[derive(Debug, Parser)]
#[command(name = "obsidian-tidy")]
#[command(
    version,
    about = "🚀 Blazingly fast Obsidian vault linter",
    long_about = None
)]
pub struct Cli {
    /// Path to Obsidian vault
    #[arg(long, value_name = "DIRECTORY", default_value = default_path().into_os_string())]
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
    Check {
        /// Use cache
        #[arg(long)]
        ignore_cache: bool,
    },

    /// Initialization of config for linter
    Init {
        #[arg(long)]
        override_config: bool,

        #[arg(long, value_enum, default_value_t = Template::Standart)]
        template: Template,
    },
}

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum Template {
    All,
    Standart,
    Empty,
}
