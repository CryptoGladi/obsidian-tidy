use super::ENV_NAME;
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// Builder for [`Logger`]
pub struct LoggerBuilder {
    /// Filter
    pub(crate) filter: EnvFilter,

    /// Enable input to stdout
    pub(crate) stdout: bool,

    /// Enable input to file
    pub(crate) file: bool,

    /// Path directory to save logs
    pub(crate) path: PathBuf,
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        let filter = EnvFilter::try_from_env(ENV_NAME).unwrap_or(EnvFilter::new("off"));
        let logs_dir = directories().logs_dir();

        Self {
            path: logs_dir,
            filter,
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

    pub fn path(mut self, value: PathBuf) -> Self {
        self.path = value;
        self
    }
}
