//! Crate for logging

#![forbid(clippy::print_stdout)]

mod builder;
mod logger;

pub use builder::LoggerBuilder;
pub use logger::Logger;
