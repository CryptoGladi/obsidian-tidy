//! Core logging functionality

pub mod worker_guard;

use crate::Error;
use crate::builder::LoggerBuilder;
use std::fmt::Debug;
use tracing::Subscriber;
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::rolling::Builder as RollingBuilder;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::{
    Layer,
    fmt::{MakeWriter, format::FmtSpan, writer::BoxMakeWriter},
    prelude::*,
    registry::LookupSpan,
};

pub use worker_guard::WorkerGuard;

/// A configured logger instance.
///
/// This struct holds a [`WorkerGuard`] that ensures buffered log records
/// are flushed to disk when dropped. **Keep an instance of [`Logger`] alive
/// for the duration of your program** to avoid losing log data.
///
/// # Example
///
/// ```no_run
/// use obsidian_tidy_logger::LoggerBuilder;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let _guard = LoggerBuilder::default()
///         .build()?
///         .init()?; // _guard must live until program exit
///
///     tracing::info!("Logs will be flushed when _guard is dropped");
///     Ok(())
/// }
/// ```
#[must_use = "Logger must be stored to keep WorkerGuard alive and ensure logs are flushed"]
pub struct Logger {
    guard: Option<WorkerGuard>,
    registry: Box<dyn Subscriber + Sync + Send>,
}

impl Logger {
    /// Install this logger as the global tracing subscriber.
    ///
    /// # Thread Safety
    ///
    /// This function is not thread-safe. Call it from `main()` before spawning
    /// threads or initializing async runtimes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let _guard = LoggerBuilder::default()
    ///         .build()?
    ///         .init()?; // Installs global subscriber
    ///
    ///     // Now tracing macros will work throughout the application
    ///     tracing::info!("Global logging is active");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// - `Ok(Some(guard))` if file logging is enabled — **store this guard**
    ///   until program exit to ensure logs are flushed.
    /// - `Ok(None)` if file logging is disabled — no guard needed.
    /// - `Err([Error::Init])` if a global subscriber is already set.
    #[track_caller]
    pub fn init(self) -> Result<Option<WorkerGuard>, Error> {
        tracing::subscriber::set_global_default(self.registry)?;

        Ok(self.guard)
    }

    /// Decompose this [`Logger`] into its components for advanced use cases.
    ///
    /// This is primarily useful for testing or custom subscriber setups
    /// where you need direct access to the `Subscriber` and `WorkerGuard`.
    ///
    /// # Warning
    ///
    /// If you use this method, you are responsible for:
    /// 1. Installing the subscriber via [`tracing::subscriber::set_global_default`]
    /// 2. Keeping the [`WorkerGuard`] alive to ensure logs are flushed
    ///
    /// Prefer [`Logger::init()`] for normal usage.
    #[must_use]
    pub fn into_components(self) -> (Box<dyn Subscriber + Send + Sync>, Option<WorkerGuard>) {
        (self.registry, self.guard)
    }
}

/// Builds a configured [`Logger`] from this builder.
impl LoggerBuilder {
    /// Internal helper: create the console output layer.
    ///
    /// Configured for clean, user-facing output:
    /// - No timestamps, targets, or span lifecycle events
    /// - Compact format with log levels only
    fn console_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::layer()
            .compact()
            .without_time()
            .with_target(false)
            .with_level(true)
            .with_span_events(FmtSpan::NONE)
            .with_writer(std::io::stdout)
            .with_filter(self.console_filter)
    }

    /// Internal helper: create the file output layer.
    ///
    /// Configured for detailed, machine-parseable diagnostics:
    /// - Full timestamps, targets, line numbers, and file paths
    /// - No ANSI colors (for log file compatibility)
    /// - Structured span context for debugging
    fn file_layer<S, W>(&self, writer: W) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
        W: for<'writer> MakeWriter<'writer> + 'static,
    {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_ansi(false)
            .with_writer(writer)
            .with_target(true)
            .with_line_number(true)
            .with_file(true)
            .with_filter(self.file_filter)
    }

    /// Internal helper: build the non-blocking file output components.
    ///
    /// Creates a [`RollingFileAppender`] configured with the builder's settings,
    /// wraps it in a non-blocking layer, and returns both the writer and the
    /// [`WorkerGuard`] that must be kept alive to ensure logs are flushed.
    ///
    /// # Returns
    ///
    /// - `Ok((NonBlocking, WorkerGuard))` on success
    /// - `Err([Error::Appender])` if file appender initialization fails
    fn build_file_output(&self) -> Result<(NonBlocking, WorkerGuard), Error> {
        let mut builder = RollingBuilder::new()
            .filename_suffix(&self.filename_suffix)
            .filename_prefix(&self.filename_prefix);

        if let Some(latest_symlink) = &self.latest_symlink {
            builder = builder.latest_symlink(latest_symlink);
        }

        let file_appender = builder
            .max_log_files(self.max_log_files)
            .rotation(Rotation::DAILY)
            .build(&self.path)?;

        let (writer, guard) = tracing_appender::non_blocking(file_appender);

        let guard = WorkerGuard::new(guard);

        Ok((writer, guard))
    }

    /// Build a configured `Logger` without installing it globally.
    ///
    /// This method validates configuration and constructs the subscriber,
    /// but does not call `set_global_default`. Use this for:
    /// - Testing with [`tracing::subscriber::with_default`]
    /// - Custom subscriber composition
    /// - Deferred initialization
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_logger::LoggerBuilder;
    ///
    /// let logger = LoggerBuilder::default()
    ///     .file(false) // Skip file I/O in tests
    ///     .build()
    ///     .expect("Failed to build logger");
    /// ```
    ///
    /// # Other
    ///
    /// Use [`Logger::into_components()`] for testing
    pub fn build(self) -> Result<Logger, Error> {
        let registry = tracing_subscriber::registry();

        let (writer, guard) = if self.file {
            let (writer, guard) = self.build_file_output()?;

            (BoxMakeWriter::new(writer), Some(guard))
        } else {
            (BoxMakeWriter::new(std::io::sink), None)
        };

        let console_layer = self.console_layer();
        let file_layer = self.file_layer(writer);

        let registry = registry
            .with(self.stdout.then_some(console_layer))
            .with(self.file.then_some(file_layer));

        Ok(Logger {
            guard,
            registry: Box::new(registry),
        })
    }
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logger")
            .field("guard", &self.guard)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn build_without_file_does_not_create_files() {
        let temp_dir = TempDir::new().unwrap();

        let logger = LoggerBuilder::default()
            .path(temp_dir.path())
            .file(false)
            .stdout(false)
            .build()
            .expect("build should succeed when file logging is disabled");

        let entries: Vec<_> = temp_dir
            .path()
            .read_dir()
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();

        assert!(
            entries.is_empty(),
            "No files should be created when file logging is disabled"
        );

        assert!(logger.guard.is_none());
    }

    #[test]
    fn build_with_file_creates_appender() {
        let temp_dir = TempDir::new().unwrap();

        let logger = LoggerBuilder::default()
            .path(temp_dir.path())
            .file(true)
            .filename_prefix("test-app")
            .unwrap()
            .filename_suffix("log")
            .unwrap()
            .build()
            .expect("build should succeed with valid path");

        assert!(logger.guard.is_some());
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn build_with_zero_max_log_files() {
        let temp_dir = TempDir::new().unwrap();

        let logger = LoggerBuilder::default()
            .path(temp_dir.path())
            .file(true)
            .max_log_files(0)
            .build()
            .expect("failed build");

        assert!(logger.guard.is_some());
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn build_fails_with_invalid_path() {
        let logger = LoggerBuilder::default()
            .path("/root/forbidden_path_xyz_12345")
            .file(true)
            .build();

        assert!(
            logger.unwrap_err().is_appender(),
            "build should fail with unwritable path"
        );
    }

    #[test]
    fn impl_debug() {
        let logger = LoggerBuilder::default()
            .file(false)
            .stdout(false)
            .build()
            .unwrap();

        let debug_output = format!("{logger:?}");
        assert!(debug_output.contains("Logger"));
    }
}
