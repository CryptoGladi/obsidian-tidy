use tracing_appender::non_blocking::WorkerGuard as TracingWorkerGuard;

/// A RAII guard that ensures buffered log records are flushed to disk.
///
/// This is a thin wrapper around [`tracing_appender::non_blocking::WorkerGuard`]
/// that provides a stable public API boundary for `obsidian-tidy-logger`.
///
/// # Important
///
/// **Keep this guard alive for the duration of your program.** When dropped,
/// it signals the background logging thread to flush and shut down. Dropping
/// it prematurely may result in lost log messages.
///
/// # Usage
///
/// ```no_run
/// use obsidian_tidy_logger::LoggerBuilder;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let _guard = LoggerBuilder::default()
///         .build()?
///         .init()?; // _guard must live until program exit
///
///     tracing::info!("Logging is active");
///     Ok(())
/// }
/// ```
///
/// [`Logger::init`]: crate::Logger::init
#[derive(Debug)]
#[expect(unused)]
#[must_use = "This guard must be kept alive to ensure logs are flushed to disk"]
pub struct WorkerGuard(TracingWorkerGuard);

impl WorkerGuard {
    /// Creates a new `WorkerGuard` from a [`TracingWorkerGuard`].
    ///
    /// This constructor is primarily for internal use. Most users should
    /// obtain a `WorkerGuard` via [`Logger::init`]
    ///
    /// [`Logger::init`]: crate::Logger::init
    #[inline]
    pub const fn new(worker_guard: TracingWorkerGuard) -> Self {
        Self(worker_guard)
    }
}
