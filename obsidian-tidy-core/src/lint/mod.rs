//! Module for trait lints

mod lints;
mod shared_error_lint;
mod smart_pointer;
mod toggleable_lint;

use crate::Vault;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Range, sync::Arc};

pub use lints::Lints;
pub use lints::serde::{InnerLints, LintsSeed};
pub use shared_error_lint::SharedErrorLint;
pub use toggleable_lint::ToggleableLint;

pub type DynLint<E> = Arc<dyn Lint<Error = E>>;

/// Trait Lint
pub trait Lint: Send + Sync {
    /// Error while work Lint
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

#[derive(Debug, Default, Clone)]
pub struct Content {
    #[allow(dead_code)]
    vault: Vault,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Violation {
    pub message: String,
    pub location: Range<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestLint;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn partial_eq() {
        let lint1 = TestLint::new("lint1", "", Category::Heading);
        let lint1_same = TestLint::new("lint1", "", Category::Heading);

        let lint2 = TestLint::new("lint2", "", Category::Heading);
        let lint2_same = TestLint::new("lint2", "", Category::Heading);

        assert_eq!(lint1, lint1_same);
        assert_eq!(lint2, lint2_same);

        assert_ne!(lint1, lint2);
        assert_ne!(lint1_same, lint2_same);
    }
}
