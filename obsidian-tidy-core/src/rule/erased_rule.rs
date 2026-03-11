use crate::{
    Note,
    rule::{Content, Rule, RuleMetadata, Violation},
};

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
pub trait ErasedRule: RuleMetadata {
    fn check(
        &self,
        content: &Content,
        note: &Note,
    ) -> Result<Vec<Violation>, Box<dyn std::error::Error>>;
}

impl<R> ErasedRule for R
where
    R: Rule,
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
        let erased_rule: Box<dyn ErasedRule> = Box::new(test_rule);

        erased_rule
            .check(&Content::default(), &Note::default())
            .unwrap();
    }

    #[test]
    fn check_with_error() {
        let error_rule = ErrorRule;
        let result = error_rule.check(&Content::default(), &Note::default());

        assert!(result.is_err());
    }
}
