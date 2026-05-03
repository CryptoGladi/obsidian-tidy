//! Configuration builder for [`Logger`].
//!
//! Use [`LoggerBuilder`] to customize logging behavior before initialization.
//!
//! [`Logger`]: crate::Logger

use crate::Error;
use obsidian_tidy_core::directories::directories;
use std::path::PathBuf;
use tracing_subscriber::filter::LevelFilter;

/// Builder for configuring and initializing a [`Logger`].
///
/// # Example
///
/// ```no_run
/// use obsidian_tidy_logger::LoggerBuilder;
/// use tracing_subscriber::filter::LevelFilter;
///
/// let _guard = LoggerBuilder::default()
///     .stdout(true)
///     .file(true)
///     .console_filter(LevelFilter::INFO)
///     .file_filter(LevelFilter::TRACE)
///     .filename_prefix("my-app")
///     .expect("filename prefix is invalid")
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
    pub(crate) console_filter: LevelFilter,

    /// Log level filter for file output.
    pub(crate) file_filter: LevelFilter,

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
    /// | `console_filter` | [`LevelFilter::INFO`] |
    /// | `file_filter` | [`LevelFilter::DEBUG`] |
    /// | `stdout` | `true` |
    /// | `file` | `true` |
    /// | `path` | [`obsidian_tidy_core::directories::directories`] |
    /// | `filename_prefix` | `""` |
    /// | `filename_suffix` | `"log"` |
    /// | `max_log_files` | `10` |
    /// | `latest_symlink` | `latest.log` (in Unix system) |
    fn default() -> Self {
        let logs_dir = directories().logs_dir();

        Self {
            path: logs_dir,
            console_filter: LevelFilter::INFO,
            file_filter: LevelFilter::DEBUG,
            stdout: true,
            file: true,
            filename_prefix: String::from(""),
            filename_suffix: String::from("log"),
            max_log_files: 10,

            // For creating symlink in Windows typically requiring
            // administrator privileges
            latest_symlink: if cfg!(unix) {
                Some(String::from("latest.log"))
            } else {
                None
            },
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
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    /// use tracing_subscriber::filter::LevelFilter;
    ///
    /// let builder = LoggerBuilder::default()
    ///     .console_filter(LevelFilter::WARN); // Only warnings and errors on console
    /// ```
    #[must_use]
    pub const fn console_filter(mut self, console_filter: LevelFilter) -> Self {
        self.console_filter = console_filter;
        self
    }

    /// Set the log level filter for file output.
    ///
    /// Events below this level will not be written to log files.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    /// use tracing_subscriber::filter::LevelFilter;
    ///
    /// let builder = LoggerBuilder::default()
    ///     .file_filter(LevelFilter::TRACE); // Full verbosity in files
    /// ```
    #[must_use]
    pub const fn file_filter(mut self, file_filter: LevelFilter) -> Self {
        self.file_filter = file_filter;
        self
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
    pub fn filename_prefix(mut self, filename_prefix: impl Into<String>) -> Result<Self, Error> {
        let f = filename_prefix.into();

        if f.contains('/') || f.contains('\\') {
            return Err(Error::InvalidFilenamePrefix(f));
        }

        self.filename_prefix = f;
        Ok(self)
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
    pub fn filename_suffix(mut self, filename_suffix: impl Into<String>) -> Result<Self, Error> {
        let f = filename_suffix.into();

        if f.contains('/') || f.contains('\\') {
            return Err(Error::InvalidFilenameSuffix(f));
        }

        self.filename_suffix = f;
        Ok(self)
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
            .console_filter(LevelFilter::WARN)
            .file_filter(LevelFilter::TRACE)
            .filename_prefix("test")
            .unwrap()
            .filename_suffix("txt")
            .unwrap()
            .max_log_files(5)
            .latest_symlink(Some(String::from("last")));

        assert!(!builder.stdout);
        assert!(builder.file);
        assert_eq!(builder.console_filter, LevelFilter::WARN);
        assert_eq!(builder.file_filter, LevelFilter::TRACE);
        assert_eq!(builder.filename_prefix, "test");
        assert_eq!(builder.filename_suffix, "txt");
        assert_eq!(builder.max_log_files, 5);
        assert_eq!(builder.latest_symlink, Some(String::from("last")));
    }

    #[test]
    fn filename_prefix_path_separators() {
        let error = LoggerBuilder::default()
            .filename_prefix("dsvfs/dsc")
            .unwrap_err();
        assert!(error.is_invalid_filename_prefix());

        let error = LoggerBuilder::default()
            .filename_prefix("dsvfs\\dsc")
            .unwrap_err();
        assert!(error.is_invalid_filename_prefix());
    }

    #[test]
    fn filename_suffix_path_separators() {
        let error = LoggerBuilder::default()
            .filename_suffix("dsvfs/dsc")
            .unwrap_err();
        assert!(error.is_invalid_filename_suffix());

        let error = LoggerBuilder::default()
            .filename_suffix("dsvfs\\dsc")
            .unwrap_err();
        assert!(error.is_invalid_filename_suffix());
    }
}
