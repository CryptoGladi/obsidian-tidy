//! Module for testing

use crate::lint::{Category, Content, Lint, Violation};
use std::convert::Infallible;

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
    ) -> Self {
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
        Ok(vec![])
    }
}
