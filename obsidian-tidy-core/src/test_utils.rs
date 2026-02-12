//! Module for testing

use crate::lint::{Category, Content, Lint, Violation};
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TestLint {
    name: String,
    description: String,
    category: Category,
}

impl TestLint {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: Category,
    ) -> TestLint {
        Self {
            name: name.into(),
            description: description.into(),
            category,
        }
    }
}

impl Lint for TestLint {
    type Error = Infallible;

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn category(&self) -> Category {
        self.category.clone()
    }

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(Vec::new())
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

        let test_lint = TestLint::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY);

        assert_eq!(test_lint.name(), TEST_NAME);
        assert_eq!(test_lint.description(), TEST_DESCRIPTION);
        assert_eq!(test_lint.category(), TEST_CATEGORY);
    }
}
