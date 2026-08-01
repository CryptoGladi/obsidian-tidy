//! Core library for **obsidian-tidy** — a linting and formatting engine for
//! [Obsidian](https://obsidian.md/) vaults.
//!
//! This crate provides the foundational abstractions for defining, registering,
//! and executing rules that validate and transform Markdown notes. It is designed
//! to be the shared core between the CLI, GUI, and any future frontends.
//!
//! # Architecture Overview
//!
//! The library is built around a few key concepts:
//!
//! - **[`Rule`]**: A unit of validation. Each rule inspects a [`Note`] (or the
//!   entire [`Vault`]) and produces zero or more [`Violation`](rule::Violation)s
//!   describing issues found.
//! - **[`Content`](rule::Content)**: A parsed representation of an Obsidian vault,
//!   providing efficient access to all notes and their structure.
//! - **[`Rules`](rule::Rules)**: A named collection of [`ToggleableRule`](rule::ToggleableRule)s,
//!   supporting runtime enable/disable and serde-based configuration.
//! - **[`RuleFabric`](rule::RuleFabric)**: A factory for dynamically instantiating
//!   rules from deserialized configuration data. Used in conjunction with
//!   [`RuleFabricRegistry`](rule::RuleFabricRegistry) for plugin-style extensibility.
//!
//! # Quick Start
//!
//! ```rust
//! use obsidian_tidy_core::rule::{Category, Rules, Rule, ToggleableRule, RuleMetadata, RuleRunner};
//! use obsidian_tidy_core::rule::{Content, Violation};
//! use obsidian_tidy_core::Note;
//! use std::convert::Infallible;
//! use serde::Serialize;
//!
//! // 1. Define a rule
//! #[derive(Debug, Default, Serialize)]
//! struct NoEmptyHeadings;
//!
//! impl RuleMetadata for NoEmptyHeadings {
//!     fn name(&self) -> &str { "no-empty-headings" }
//!     fn description(&self) -> &str { "Flags headings with no content below them." }
//!     fn category(&self) -> Category { Category::Heading }
//! }
//!
//! impl RuleRunner for NoEmptyHeadings {
//!     type Error = Infallible;
//!     fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
//!         // ... rule logic ...
//!         Ok(vec![])
//!     }
//! }
//!
//! // 2. Register it in a collection
//! let mut rules = Rules::new();
//! rules.add(ToggleableRule::new(NoEmptyHeadings, true).into_erased());
//!
//! assert!(rules.contains("no-empty-headings"));
//! ```
//!
//! # Module Overview
//!
//! - [`directories`]: Resolves platform-specific paths for config, data, and logs
//!   using the [`directories`](https://crates.io/crates/directories) crate.
//! - [`rule`]: The rule engine — traits, collections, type-erased fabrics, and
//!   serde integration for heterogeneous rule registries.
//! - [`test_utils`]: Utilities for testing rules and fabrics. Only meaningful in
//!   test builds; re-exported for downstream crates writing integration tests.
//!
//! # Extensibility
//!
//! [`RuleFabric`](rule::RuleFabric) trait and can coexist in a single
//! [`RuleFabricRegistry`](rule::RuleFabricRegistry).
//!
//! # Design Principles
//!
//! - **Zero-cost abstractions**: Metadata accessors return `&str`, which the
//!   compiler optimizes to direct static pointers for built-in rules.
//! - **No hidden allocations**: The serde pipeline streams directly into rule
//!   factories via `erased_serde` — no intermediate `RawValue` buffering.
//! - **Explicit over implicit**: Duplicate rule names are detected and reported
//!   at registration time, not silently overwritten.
//! - **Format agnostic**: Configuration works with JSON, YAML, TOML, or any
//!   serde-compatible format out of the box.
//!
//! [`Rule`]: rule::Rule
//! [`Content`]: rule::Content
//! [`Rules`]: rule::Rules
//! [`RuleFabric`]: rule::RuleFabric

#![forbid(clippy::print_stdout)]
#![deny(unsafe_code)]

pub mod directories;
pub mod rule;

/// Utilities for testing rules and fabrics.
///
/// This module is intended for use in integration tests of downstream crates.
/// In non-test builds, unused items will emit a warning rather than an error.
#[cfg(any(test, doc, debug_assertions))]
pub mod test_utils;

/// Utilities for benchmarks.
///
/// Test rules for benchmarks are located here.
#[cfg_attr(not(test), warn(unused))]
pub mod bench_utils;

/// A parsed Obsidian vault containing a collection of [`Note`]s.
///
/// This is a type alias over [`obsidian_parser::vault::Vault`] specialized
/// for the in-memory note representation used by this crate.
pub type Vault = obsidian_parser::vault::Vault<Note>;

/// An in-memory representation of a single Obsidian note.
///
/// Type alias for convenience, hiding the underlying parser implementation.
pub type Note = obsidian_parser::note::note_in_memory::NoteInMemory;

/// Error type produced when parsing or processing a [`Note`] fails.
pub type NoteError = obsidian_parser::note::note_in_memory::Error;
