use crate::{
    Note,
    rule::{Category, Content, RuleFactory, RuleMetadata, RuleRunner, Violation},
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct TestRule {
    name: String,
    description: String,
    category: Category,
    check_result: Vec<Violation>,
}

impl Default for TestRule {
    fn default() -> Self {
        Self {
            name: "test-rule".to_string(),
            description: "A test rule".to_string(),
            category: Category::Content,
            check_result: Vec::new(),
        }
    }
}

impl TestRule {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: Category,
        check_result: impl IntoIterator<Item = Violation>,
    ) -> TestRule {
        Self {
            name: name.into(),
            description: description.into(),
            category,
            check_result: check_result.into_iter().collect(),
        }
    }
}

impl RuleMetadata for TestRule {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn category(&self) -> Category {
        self.category
    }
}

impl RuleRunner for TestRule {
    type Error = Infallible;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(self.check_result.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRuleFactory {
    name: String,
}

impl Default for TestRuleFactory {
    fn default() -> Self {
        // It is MOCK!
        let test_rule = TestRule::default();

        Self {
            name: test_rule.name,
        }
    }
}

impl TestRuleFactory {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl RuleFactory for TestRuleFactory {
    type Rule = TestRule;
    type Data = TestRule;
    type Error = Infallible;

    fn id(&self) -> &str {
        &self.name
    }

    fn create_by_serde(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
        Ok(data)
    }

    fn create_default(&self) -> Option<Self::Rule> {
        Some(TestRule::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);

        assert_eq!(test_rule.name(), TEST_NAME);
        assert_eq!(test_rule.description(), TEST_DESCRIPTION);
        assert_eq!(test_rule.category(), TEST_CATEGORY);
    }
}
