//! Config for logger

use clap::{Args, ValueHint};
use obsidian_tidy_core::directories::directories;
use std::{ffi::OsString, path::PathBuf};

fn default_log_dir() -> OsString {
    directories().logs_dir().into_os_string()
}

/// Config for logger
#[derive(Debug, Args, Clone)]
pub struct LoggerConfig {
    /// Path to directory for logs
    /// Default save to locale share data
    #[arg(long, value_name = "DIRECTORY", value_hint = ValueHint::DirPath, default_value_os = default_log_dir())]
    pub path_log: PathBuf,

    /// Log level for console
    #[arg(
        long,
        env = "OBSIDIAN_TIDY_CONSOLE_FILTER",
        value_name = "FILTER",
        default_value = "info"
    )]
    pub console_filter: String,

    /// Log level for file
    #[arg(
        long,
        env = "OBSIDIAN_TIDY_FILE_FILTER",
        value_name = "FILTER",
        default_value = "off"
    )]
    pub file_filter: String,

    /// We don't output anything to the terminal.
    #[arg(long)]
    pub quiet: bool,
}
