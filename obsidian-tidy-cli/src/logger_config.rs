//! Config for logger

use clap::{Args, ValueEnum, ValueHint};
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;

/// Log level
#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// [`LogLevel`] to [`tracing::Level`]
///
/// # Example
/// ```
/// use obsidian_tidy_cli::LogLevel;
///
/// let log_level = LogLevel::Warn;
/// let tracing_level: tracing::Level = log_level.into();
///
/// assert_eq!(tracing_level, tracing::Level::WARN);
/// ```
impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

/// [`LogLevel`] to [`tracing_subscriber::filter::LevelFilter`]
///
/// # Example
/// ```
/// use obsidian_tidy_cli::LogLevel;
/// use tracing_subscriber::filter::LevelFilter;
///
/// let log_level = LogLevel::Warn;
/// let tracing_level: LevelFilter = log_level.into();
///
/// assert_eq!(tracing_level, LevelFilter::WARN);
/// ```
impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(level: LogLevel) -> Self {
        use tracing_subscriber::filter::LevelFilter;

        match level {
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}

/// Config for logger
#[derive(Debug, Args)]
pub struct LoggerConfig {
    #[arg(long)]
    /// Enable logger
    pub enable_logger: bool,

    /// Path to directory for logs
    /// Default save to locale share data
    #[arg(long, value_name = "DIRECTORY", value_hint = ValueHint::DirPath, default_value = directories().logs_dir().into_os_string())]
    pub path_log: PathBuf,

    /// Log level
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// Enable logger to stdout
    #[arg(long)]
    pub enable_logger_stdout: bool,

    /// Enable logger to file
    #[arg(long)]
    pub enable_logger_file: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tracing_level() {
        let log_level = LogLevel::Debug;
        let tracing_level: tracing::Level = log_level.into();

        assert_eq!(tracing_level, tracing::Level::DEBUG);
    }

    #[test]
    fn from_tracing_level_filter() {
        use tracing_subscriber::filter::LevelFilter;

        let log_level = LogLevel::Debug;
        let tracing_level: LevelFilter = log_level.into();

        assert_eq!(tracing_level, LevelFilter::DEBUG);
    }
}
