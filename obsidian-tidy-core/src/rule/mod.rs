//! Module for trait rules

pub mod category;
pub mod content;
pub mod rules;
pub mod shared_error_rule;
pub mod smart_pointer;
pub mod toggleable_rule;
pub mod violation;

use crate::Note;
use std::{fmt::Debug, sync::Arc};

pub use category::Category;
pub use content::Content;
pub use rules::Rules;
pub use rules::serde::{InnerRules, RulesSeed};
pub use shared_error_rule::SharedErrorRule;
pub use toggleable_rule::ToggleableRule;
pub use violation::Violation;

/// Dyn for [`Rule`]
pub type DynRule<E> = Arc<dyn Rule<Error = E>>;

/// Trait for rule
pub trait Rule: Send + Sync {
    /// Error while work rule
    type Error: std::error::Error;

    /// **Unique** rule name
    fn name(&self) -> &str;

    /// Description rule
    fn description(&self) -> &str;

    /// Category rule
    fn category(&self) -> Category;

    /// Run check by this rule
    fn check(&self, content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error>;
}

impl<E> Debug for dyn Rule<Error = E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name())
            .field("description", &self.description())
            .field("category", &self.category())
            .finish()
    }
}

impl<E> PartialEq for dyn Rule<Error = E>
where
    E: std::error::Error,
{
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl<E> Eq for dyn Rule<Error = E> where E: std::error::Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestRule;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn partial_eq() {
        let rule1 = TestRule::new("rule1", "", Category::Heading, []);
        let rule1_same = TestRule::new("rule1", "", Category::Heading, []);

        let rule2 = TestRule::new("rule2", "", Category::Heading, []);
        let rule2_same = TestRule::new("rule2", "", Category::Heading, []);

        assert_eq!(rule1, rule1_same);
        assert_eq!(rule2, rule2_same);

        assert_ne!(rule1, rule2);
        assert_ne!(rule1_same, rule2_same);
    }
}
