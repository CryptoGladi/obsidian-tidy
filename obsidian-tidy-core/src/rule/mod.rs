//! Module for trait rules

pub mod category;
pub mod content;
pub mod erased_rule;
pub mod rule_fabric;
pub mod rules;
pub mod toggleable_rule;
pub mod violation;

use crate::Note;
use std::fmt::Debug;

pub use category::Category;
pub use content::Content;
pub use erased_rule::{ErasedRule, ErasedRuleRunner, GetErasedRule};
pub use rule_fabric::{ErasedRuleFabric, RuleFabric, RuleFabricRegistry};
pub use rules::Rules;
pub use rules::serde::RulesSeed;
pub use toggleable_rule::ToggleableRule;
pub use violation::Violation;

pub trait RuleConstMetadata: Send + Sync {
    /// **Unique** rule name
    const NAME: &'static str;

    /// Description rule
    const DESCRIPTION: &'static str;

    /// Category rule
    const CATEGORY: Category;
}

pub trait RuleMetadata: Send + Sync {
    /// **Unique** rule name
    fn name(&self) -> &str;

    /// Description rule
    fn description(&self) -> &str;

    /// Category rule
    fn category(&self) -> Category;
}

impl<RCM> RuleMetadata for RCM
where
    RCM: RuleConstMetadata + ?Sized,
{
    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        Self::DESCRIPTION
    }

    fn category(&self) -> Category {
        Self::CATEGORY
    }
}

pub trait RuleRunner: Send + Sync {
    /// Error while work rule
    type Error: std::error::Error;

    /// Run check by this rule
    fn check(&self, content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error>;
}

/// Trait for rule
pub trait Rule: RuleRunner + RuleMetadata {}

impl<R> Rule for R where R: RuleRunner + RuleMetadata {}

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
