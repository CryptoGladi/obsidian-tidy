//! Configuration builder for [`Logger`].
//!
//! Use [`LoggerBuilder`] to customize logging behavior before initialization.
//!
//! [`Logger`]: crate::Logger

use crate::Error;
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;
use tracing_subscriber::filter::{Builder as EnvFilterBuilder, EnvFilter};

// On Unix, create a 'latest.log' symlink to the current log file.
// On Windows, symlinks typically require administrator privileges, so this feature is disabled.
#[cfg(unix)]
pub const DEFAULT_LATEST_SYMLINK: Option<&str> = Some("latest.log");

#[cfg(not(unix))]
pub const DEFAULT_LATEST_SYMLINK: Option<&str> = None;

pub const DEFAULT_CONSOLE_FILTER: &str = "info";
pub const DEFAULT_FILE_FILTER: &str = "debug";

/// Builder for configuring and initializing a [`Logger`].
///
/// # Example
///
/// ```no_run
/// use obsidian_tidy_logger::LoggerBuilder;
///
/// let _guard = LoggerBuilder::default()
///     .stdout(true)
///     .file(true)
///     .console_filter("info")
///     .expect("parse console filter")
///     .file_filter("trace")
///     .expect("parse file filter")
///     .filename_prefix("my-app")
///     .max_log_files(14)
///     .build()
///     .expect("Failed to build logger")
///     .init()
///     .expect("Failed to initialize global subscriber");
/// ```
///
/// [`Logger`]: crate::Logger
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    /// Log level filter for console output.
    pub(crate) console_filter: EnvFilter,

    /// Log level filter for file output.
    pub(crate) file_filter: EnvFilter,

    /// Whether to enable console (stdout) output.
    pub(crate) stdout: bool,

    /// Whether to enable file output.
    pub(crate) file: bool,

    /// Directory path where log files will be stored.
    pub(crate) path: PathBuf,

    /// Prefix for log filenames (e.g., `"obsidian-tidy"` → `obsidian-tidy.2024-01-01.log`).
    pub(crate) filename_prefix: String,

    /// Suffix/extension for log filenames (e.g., `"log"` → `.log`).
    pub(crate) filename_suffix: String,

    /// Maximum number of rotated log files to retain.
    pub(crate) max_log_files: usize,

    pub(crate) latest_symlink: Option<String>,
}

impl Default for LoggerBuilder {
    /// Returns a builder with sensible defaults for CLI applications:
    ///
    /// | Setting | Default |
    /// |---------|---------|
    /// | `console_filter` | [`DEFAULT_CONSOLE_FILTER`] |
    /// | `file_filter` | [`DEFAULT_FILE_FILTER`] |
    /// | `stdout` | `true` |
    /// | `file` | `false` |
    /// | `path` | [`obsidian_tidy_core::directories::directories`] |
    /// | `filename_prefix` | `""` |
    /// | `filename_suffix` | `"log"` |
    /// | `max_log_files` | `10` |
    /// | `latest_symlink` | `latest.log` (in Unix system) |
    fn default() -> Self {
        let logs_dir = directories().logs_dir();

        Self {
            path: logs_dir,
            console_filter: EnvFilter::from(DEFAULT_CONSOLE_FILTER),
            file_filter: EnvFilter::from(DEFAULT_FILE_FILTER),
            stdout: true,
            file: false,
            filename_prefix: String::new(),
            filename_suffix: String::from("log"),
            max_log_files: 10,
            latest_symlink: DEFAULT_LATEST_SYMLINK.map(str::to_string),
        }
    }
}

impl LoggerBuilder {
    /// Enable or disable console (stdout) output.
    ///
    /// When disabled, no logs are written to stdout, but file logging
    /// (if enabled) continues unaffected.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().stdout(false);
    /// ```
    #[must_use]
    pub const fn stdout(mut self, enable: bool) -> Self {
        self.stdout = enable;
        self
    }

    /// Enable or disable file output.
    ///
    /// When disabled, no log files are created and no disk I/O occurs.
    /// Console logging (if enabled) continues unaffected.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().file(false);
    /// ```
    #[must_use]
    pub const fn file(mut self, enable: bool) -> Self {
        self.file = enable;
        self
    }

    /// Set the directory path for log files.
    ///
    /// The path will be created if it does not exist (during `build()`).
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().path("./logs");
    /// ```
    #[must_use]
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    /// Set the log level filter for console output.
    ///
    /// Events below this level will not be printed to stdout.
    ///
    /// Syntax in [`tracing_subscriber::filter::EnvFilter`]
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    /// use tracing_subscriber::filter::LevelFilter;
    ///
    /// let builder = LoggerBuilder::default()
    ///     .console_filter("warn") // Only warnings and errors on console
    ///     .expect("parse console filter");
    /// ```
    pub fn console_filter(mut self, console_filter: impl AsRef<str>) -> Result<Self, Error> {
        self.console_filter = EnvFilterBuilder::default().parse(console_filter)?;
        Ok(self)
    }

    /// Set the log level filter for file output.
    ///
    /// Events below this level will not be written to log files.
    ///
    /// Syntax in [`tracing_subscriber::filter::EnvFilter`]
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    /// use tracing_subscriber::filter::LevelFilter;
    ///
    /// let builder = LoggerBuilder::default()
    ///     .file_filter("trace") // Full verbosity in files
    ///     .expect("parse console filter");
    /// ```
    pub fn file_filter(mut self, file_filter: impl AsRef<str>) -> Result<Self, Error> {
        self.file_filter = EnvFilterBuilder::default().parse(file_filter)?;
        Ok(self)
    }

    /// Set the prefix for log filenames.
    ///
    /// The final filename format is: `{prefix}.{date}.{suffix}`
    /// (e.g., `"my-app"` + `"2024-01-01"` + `"log"` → `my-app.2024-01-01.log`).
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().filename_prefix("my-cli-tool");
    /// ```
    #[must_use]
    pub fn filename_prefix(mut self, filename_prefix: impl Into<String>) -> Self {
        self.filename_prefix = filename_prefix.into();
        self
    }

    /// Set the suffix/extension for log filenames.
    ///
    /// Common values: `"log"`, `"txt"`.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().filename_suffix("ndjson");
    /// ```
    #[must_use]
    pub fn filename_suffix(mut self, filename_suffix: impl Into<String>) -> Self {
        self.filename_suffix = filename_suffix.into();
        self
    }

    /// Set the maximum number of rotated log files to retain.
    ///
    /// Older files beyond this limit are automatically deleted during rotation.
    /// If `0` is supplied, logger will not remove any files.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let builder = LoggerBuilder::default().max_log_files(30);
    /// ```
    ///
    /// # Links
    ///
    /// - [`tracing_appender::rolling::Builder::max_log_files`] — full docs
    #[must_use]
    pub const fn max_log_files(mut self, max_log_files: usize) -> Self {
        self.max_log_files = max_log_files;
        self
    }

    #[must_use]
    pub fn latest_symlink(mut self, latest_symlink: Option<impl Into<String>>) -> Self {
        self.latest_symlink = latest_symlink.map(Into::into);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_method_chaining() {
        let builder = LoggerBuilder::default()
            .stdout(false)
            .file(true)
            .console_filter("info")
            .expect("parse console filter")
            .file_filter("trace")
            .expect("parse file filter")
            .filename_prefix("test")
            .filename_suffix("txt")
            .max_log_files(5)
            .latest_symlink(Some(String::from("last")));

        assert!(!builder.stdout);
        assert!(builder.file);
        assert_eq!(builder.console_filter.to_string(), "info");
        assert_eq!(builder.file_filter.to_string(), "trace");
        assert_eq!(builder.filename_prefix, "test");
        assert_eq!(builder.filename_suffix, "txt");
        assert_eq!(builder.max_log_files, 5);
        assert_eq!(builder.latest_symlink, Some(String::from("last")));
    }

    #[test]
    fn invalid_console_filter() {
        assert!(
            LoggerBuilder::default()
                .console_filter("=ds")
                .unwrap_err()
                .is_parse()
        )
    }

    #[test]
    fn invalid_file_filter() {
        assert!(
            LoggerBuilder::default()
                .file_filter("=ds")
                .unwrap_err()
                .is_parse()
        )
    }

    #[test]
    fn impl_default() {
        let _ = LoggerBuilder::default().build().unwrap();
    }
}
