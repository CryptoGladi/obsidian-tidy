//! # obsidian-tidy-config
//!
//! Configuration management for the `obsidian-tidy` project.
//!
//! This crate provides a type-safe, extensible system for loading, saving, and
//! manipulating configuration for Obsidian vault tidying rules. It enforces
//! construction invariants at compile time and supports flexible serialization
//! via `serde`.
//!
//! ## Key Features
//!
//! - **Controlled construction**: [`Config`] can only be created via [`Template`] or
//!   the internal [`ConfigLoader`], preventing invalid states.
//! - **Builder-style API**: Fluent methods like [`Config::loader()`] and
//!   [`Config::saver()`] guide users toward correct usage.
//! - **Template system**: Predefined configurations  simplify common use cases.
//!
//! ## Quick Start
//!
//! ```
//! use obsidian_tidy_config::{Config, Template};
//! use obsidian_tidy_core::rule_fabric_registry;
//! use obsidian_tidy_rules::ALL_RULES_FABRICS;
//! use std::io::Cursor;
//!
//! // Create a config from a template
//! let config = Config::new(Template::Standard);
//!
//! // Save
//! let mut buffer = Vec::new();
//! config.saver().save(&mut buffer)?;
//!
//! // Load
//! let loaded = Config::loader(&ALL_RULES_FABRICS)
//!     .load(Cursor::new(&buffer))?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! [`ConfigLoader`]: loader::ConfigLoader

pub mod config;
pub mod error;
mod loader;
mod saver;
pub mod template;

pub use config::Config;
pub use error::Error;
pub use template::Template;
