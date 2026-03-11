use crate::{
    Note,
    rule::{Category, Content, RuleConstMetadata, RuleRunner, Violation},
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Oh no...")]
    OhNo,
}

#[derive(Deserialize)]
pub struct ErrorRule;

impl RuleConstMetadata for ErrorRule {
    const NAME: &'static str = "error-rule";
    const DESCRIPTION: &'static str = "A rule that always returns an error";
    const CATEGORY: Category = Category::Content;
}

impl RuleRunner for ErrorRule {
    type Error = self::Error;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Err(Error::OhNo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::rule_fabric::GetFabricFromRuleConstMetadata;

    #[test]
    fn fabric() {
        let _fabric = ErrorRule::fabric();
    }

    #[test]
    fn check() {
        let error_rule = ErrorRule;
        let result = error_rule.check(&Content::default(), &Note::default());

        assert!(matches!(result, Err(Error::OhNo)));
    }
}
