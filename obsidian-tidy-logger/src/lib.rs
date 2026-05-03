//! # obsidian-tidy-logger
//!
//! A flexible, high-performance logging backend for the `obsidian-tidy` CLI tool,
//! built on [`tracing`] and [`tracing-subscriber`].
//!
//! ## Features
//!
//! - **Dual output**: Send logs to both console (user-facing) and file (diagnostic)
//!   with independent log-level filters.
//! - **Console-optimized output**: Clean, `println!`-style messages.
//! - **File-optimized output**: Full structured logs with timestamps, targets,
//!   line numbers, and span context for debugging.
//! - **Log rotation**: Automatic daily rotation with configurable retention.
//! - **Zero-cost when disabled**: If file logging is off, no files are created
//!   and no syscalls are performed.
//! - **Test-friendly**: Build subscribers without touching global state via
//!   [`LoggerBuilder::build()`].
//!
//! ## Quick Start
//!
//! ```no_run
//! use obsidian_tidy_logger::LoggerBuilder;
//! use tracing_subscriber::filter::LevelFilter;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize logging: console shows INFO+, file shows DEBUG+
//!     let _guard = LoggerBuilder::default()
//!         .console_filter(LevelFilter::INFO)
//!         .file_filter(LevelFilter::DEBUG)
//!         .filename_prefix("obsidian-tidy")?
//!         .max_log_files(7)
//!         .build()?
//!         .init()?; // _guard must live until program exit
//!
//!     tracing::info!("Application started");
//!     tracing::debug!("Verbose diagnostic info");
//!
//!     // ... your application logic ...
//!
//!     Ok(()) // _guard is dropped here, ensuring logs are flushed
//! }
//! ```
//!
//! ## Testing
//!
//! Use `build()` to create a subscriber without installing it globally:
//!
//! ```
//! use obsidian_tidy_logger::LoggerBuilder;
//! use tracing_subscriber::filter::LevelFilter;
//!
//! // #[test]
//! fn test_my_feature() {
//!     let (subscriber, _guard) = LoggerBuilder::default()
//!         .console_filter(LevelFilter::OFF) // silence console in tests
//!         .file(false)                      // skip file I/O
//!         .build()
//!         .unwrap()
//!         .into_components();
//!
//!     tracing::subscriber::with_default(subscriber, || {
//!         // Your test code here — logs are captured but not printed
//!         tracing::debug!("This won't appear in console");
//!     });
//! }
//! ```
//!
//! ## Important Notes
//!
//! - **Global state**: [`Logger::init()`] installs a global tracing subscriber.
//!   Call it **once** at the start of `main()`, before spawning threads or
//!   async runtimes.
//! - **Keep [`Logger`] alive**: The returned [`Logger`] holds a [`WorkerGuard`] that
//!   ensures buffered logs are flushed to disk. Store it for the duration of
//!   your program (e.g., `let _guard = ...;` in `main()`).
//! - **Thread safety**: The global subscriber is thread-safe after initialization.
//!
//! [`tracing`]: https://docs.rs/tracing
//! [`tracing-subscriber`]: https://docs.rs/tracing-subscriber

#![forbid(clippy::print_stdout)]

pub mod builder;
pub mod error;
pub mod logger;

pub use builder::LoggerBuilder;
pub use error::Error;
pub use logger::{Logger, WorkerGuard};
