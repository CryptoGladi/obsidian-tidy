//! Module for build Logger

use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;
use tracing_subscriber::filter::LevelFilter;

/// Builder for [`Logger`]
pub struct LoggerBuilder {
    /// Filter
    pub(crate) filter: LevelFilter,

    /// Enable input to stdout
    pub(crate) stdout: bool,

    /// Enable input to file
    pub(crate) file: bool,

    /// Path directory to save logs
    pub(crate) path: PathBuf,
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        let logs_dir = directories().logs_dir();

        Self {
            path: logs_dir,
            filter: LevelFilter::INFO,
            stdout: true,
            file: true,
        }
    }
}

impl LoggerBuilder {
    /// Enable input log to stdout
    pub fn stdout(mut self, enable: bool) -> Self {
        self.stdout = enable;
        self
    }

    /// Enable input log to file
    pub fn file(mut self, enable: bool) -> Self {
        self.file = enable;
        self
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }

    pub fn filter(mut self, filter: LevelFilter) -> Self {
        self.filter = filter;
        self
    }
}
