use crate::{
    Note,
    rule::{Category, Content, Rule, RuleMetadata, RuleRunner, Violation},
};
use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;
use std::fmt::Debug;

/// Type erasing for [`Rule`]
///
/// # Example
///
/// This code is don't work:
/// ```compile_fail
/// use obsidian_tidy_core::rule::Rule;
/// # use obsidian_tidy_core::test_utils::TestRule;
///
/// let test_rule = TestRule::default();
/// let erased_rule: Box<dyn Rule> = Box::new(test_rule);
/// ```
///
/// But this is work:
/// ```
/// use obsidian_tidy_core::rule::{Rule, ErasedRule};
/// # use obsidian_tidy_core::test_utils::TestRule;
///
/// let test_rule = TestRule::default();
/// let erased_rule: Box<dyn ErasedRule> = Box::new(test_rule);
/// ```
pub trait ErasedRuleRunner {
    fn check(
        &self,
        content: &Content,
        note: &Note,
    ) -> Result<Vec<Violation>, Box<dyn std::error::Error>>;
}

impl<R> ErasedRuleRunner for R
where
    R: RuleRunner,
    R::Error: 'static,
{
    fn check(
        &self,
        content: &Content,
        note: &Note,
    ) -> Result<Vec<Violation>, Box<dyn std::error::Error>> {
        let result = self
            .check(content, note)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(result)
    }
}

// Don't delete ErasedSerialize!
pub trait ErasedRule: ErasedRuleRunner + RuleMetadata + ErasedSerialize {}

static_assertions::assert_obj_safe!(ErasedRuleRunner, ErasedRule);
static_assertions::assert_trait_sub_all!(ErasedRule: ErasedRuleRunner, RuleMetadata, ErasedSerialize);

erased_serde::serialize_trait_object!(ErasedRule);

impl<R> ErasedRule for R where R: ErasedRuleRunner + RuleMetadata + ErasedSerialize {}

impl RuleMetadata for Box<dyn ErasedRule> {
    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn description(&self) -> &str {
        self.as_ref().description()
    }

    fn category(&self) -> Category {
        self.as_ref().category()
    }
}

pub trait GetErasedRule {
    fn into_erased(self) -> Box<dyn ErasedRule>;
}

impl<R> GetErasedRule for R
where
    R: Rule + Serialize + 'static,
{
    fn into_erased(self) -> Box<dyn ErasedRule> {
        Box::new(self) as Box<dyn ErasedRule>
    }
}

impl<R> From<R> for Box<dyn ErasedRule>
where
    R: GetErasedRule,
{
    fn from(value: R) -> Self {
        value.into_erased()
    }
}

impl Debug for dyn ErasedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErasedRule")
            .field("name", &self.name())
            .field("description", &self.description())
            .field("category", &self.category())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{ErrorRule, TestRule};

    #[test]
    fn metadata() {
        let test_rule = TestRule::default();
        let erased_rule: Box<dyn ErasedRule> = Box::new(test_rule.clone());

        assert_eq!(erased_rule.name(), test_rule.name());
        assert_eq!(erased_rule.description(), test_rule.description());
        assert_eq!(erased_rule.category(), test_rule.category());
    }

    #[test]
    fn check() {
        let test_rule = TestRule::default();
        let erased_rule: Box<dyn ErasedRuleRunner> = Box::new(test_rule);

        erased_rule
            .check(&Content::default(), &Note::default())
            .unwrap();
    }

    #[test]
    fn check_with_error() {
        let error_rule = ErrorRule;
        let result = ErasedRuleRunner::check(&error_rule, &Content::default(), &Note::default());

        assert!(result.is_err());
    }
}
