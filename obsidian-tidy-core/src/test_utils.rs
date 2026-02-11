//! Module for testing

use crate::lint::{Category, Content, Lint, Violation};

pub(crate) struct TestLint {
    name: String,
    category: Category,
}

impl TestLint {
    pub fn new(name: impl Into<String>, category: Category) -> Self {
        Self {
            name: name.into(),
            category,
        }
    }
}

impl Lint for TestLint {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        unimplemented!()
    }

    fn category(&self) -> Category {
        self.category.clone()
    }

    fn check(&self, _content: &Content) -> Vec<Violation> {
        unimplemented!()
    }
}
