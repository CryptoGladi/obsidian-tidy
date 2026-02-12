//! Module for trait lints

mod boxed_error_lint;
mod lints;
mod toggleable_lint;

use crate::Vault;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Range, sync::Arc};

pub use boxed_error_lint::{BoxedErrorLint, WrappedAnyhowError};
pub use lints::Lints;
pub use lints::serde::{InnerLints, LintsSeed};
pub use toggleable_lint::ToggleableLint;

pub type DynLint<E> = Arc<dyn Lint<Error = E>>;

/// Trait Lint
pub trait Lint: Send + Sync {
    type Error: std::error::Error;

    /// **Unique** lint name
    fn name(&self) -> &str;

    /// Description lint
    fn description(&self) -> &str;

    /// Category lint
    fn category(&self) -> Category;

    /// Run check by lint
    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error>;
}

impl<E> Debug for dyn Lint<Error = E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lint")
            .field("name", &self.name())
            .field("description", &self.description())
            .field("category", &self.category())
            .finish()
    }
}

impl<E> PartialEq for dyn Lint<Error = E>
where
    E: std::error::Error,
{
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<E> Eq for dyn Lint<Error = E> where E: std::error::Error {}

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
