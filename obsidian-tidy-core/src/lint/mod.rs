//! Module for trait lints

mod lints;
mod toggleable_lint;

use crate::Vault;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Range, sync::Arc};
use thiserror::Error;

pub use lints::Lints;
pub use lints::serde::{InnerLints, LintsSeed};
pub use toggleable_lint::ToggleableLint;

pub type DynLint = Arc<dyn Lint>;

/// Trait Lint
pub trait Lint: Send + Sync {
    /// **Unique** lint name
    fn name(&self) -> &str;

    /// Description lint
    fn description(&self) -> &str;

    /// Category lint
    fn category(&self) -> Category;

    /// Run check by lint
    fn check(&self, content: &Content) -> Vec<Violation>;
}

impl Debug for dyn Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lint")
            .field("name", &self.name())
            .field("description", &self.description())
            .field("category", &self.category())
            .finish()
    }
}

impl PartialEq for dyn Lint {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Lint {}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Found duplicate name: {0}")]
    DuplicateName(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Yaml,
    Heading,
    Content,
    Spacing,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Content {
    #[allow(dead_code)]
    vault: Vault,
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub message: String,
    pub location: Range<usize>,
}
