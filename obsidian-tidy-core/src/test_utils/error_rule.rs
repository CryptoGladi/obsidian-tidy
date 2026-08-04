use crate::{
    Note,
    rule::{
        Category, Content, RuleMetadata, RuleRunner, Violation,
        factory::impl_rule_factory_with_serde,
    },
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Oh no...")]
    OhNo,
}

#[derive(Deserialize, Serialize, Default)]
pub struct ErrorRule;

impl RuleMetadata for ErrorRule {
    fn name(&self) -> &str {
        "error-rule"
    }

    fn description(&self) -> &str {
        "A rule that always returns an error"
    }

    fn category(&self) -> Category {
        Category::Content
    }
}

impl RuleRunner for ErrorRule {
    type Error = self::Error;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Err(Error::OhNo)
    }
}

pub struct ErrorFactory;
impl_rule_factory_with_serde!(ErrorFactory, ErrorRule, "error-rule");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let error_rule = ErrorRule;
        let result = error_rule.check(&Content::default(), &Note::default());

        assert!(matches!(result, Err(Error::OhNo)));
    }
}
