//! Error types for the `obsidian-tidy-logger` crate.
//!
//! This module defines the [`enum@Error`] enum, which represents all recoverable
//! errors that can occur during logger configuration and initialization.

use derive_more::IsVariant;
use thiserror::Error;

/// Errors that can occur during logger setup.
///
/// This enum provides a unified error type for the logger initialization pipeline,
/// combining errors from both the file appender setup and the global tracing
/// subscriber installation.
///
/// # Example
///
/// ```no_run
/// use obsidian_tidy_logger::{LoggerBuilder, Error};
///
/// fn setup_logging() -> Result<(), Error> {
///     let _guard = LoggerBuilder::default()
///         .build()?
///         .init()?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Error, IsVariant)]
#[non_exhaustive]
pub enum Error {
    /// Failed to initialize the file-based log appender.
    ///
    /// This error occurs when:
    /// - The log directory cannot be created or accessed
    /// - File permissions are insufficient
    /// - The filename prefix/suffix contains invalid characters
    /// - Disk space is exhausted
    #[error("Failed to initialize file appender: {0}")]
    Appender(#[from] tracing_appender::rolling::InitError),

    /// Failed to install the logger as the global tracing subscriber.
    ///
    /// This error typically occurs when:
    /// - A global subscriber has already been set (e.g., by a test or dependency)
    /// - The logger is initialized from multiple threads concurrently
    #[error("Error while initializing logging: {0}")]
    Init(#[from] tracing::subscriber::SetGlobalDefaultError),

    /// Invalid filename prefix: must not contain path separators
    #[error("Invalid filename prefix `{0}`: must not contain path separators")]
    InvalidFilenamePrefix(String),

    /// Invalid filename suffix: must not contain path separators
    #[error("Invalid filename suffix: `{0}`: must not contain path separators")]
    InvalidFilenameSuffix(String),
}
