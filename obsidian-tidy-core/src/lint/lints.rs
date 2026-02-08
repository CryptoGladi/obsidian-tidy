mod serde;

use super::{Error, ToggleableLint};
use std::{collections::HashSet, ops::Deref};
use tracing::{instrument, trace};

#[derive(Debug)]
pub struct Lints(Vec<ToggleableLint>);

impl Lints {
    #[instrument(ret)]
    fn check_unique_name(lints: &[ToggleableLint]) -> Result<(), Error> {
        trace!("Check unique name");

        let mut names = HashSet::with_capacity(lints.len());
        let iter = lints.iter().map(|lint| lint.name());

        for name in iter {
            if !names.insert(name) {
                return Err(Error::DuplicateName(name.to_string()));
            }
        }

        Ok(())
    }

    pub fn new(lints: Vec<ToggleableLint>) -> Result<Self, Error> {
        Self::check_unique_name(&lints)?;
        Ok(Self(lints))
    }
}

impl Deref for Lints {
    type Target = Vec<ToggleableLint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::{Category, Content, Lint, Violation};

    struct TestLint {
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

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";
        let lint1 = ToggleableLint::new(Box::new(TestLint::new(name, Category::Custom)));
        let lint2 = ToggleableLint::new(Box::new(TestLint::new(name, Category::Custom)));

        let lints = Lints::new(vec![lint1, lint2]);

        assert_eq!(lints.err(), Some(Error::DuplicateName(name.to_string())))
    }

    #[test]
    fn new() {
        let lint1 = ToggleableLint::new(Box::new(TestLint::new("Lint1", Category::Content)));
        let lint2 = ToggleableLint::new(Box::new(TestLint::new("Lint2", Category::Content)));

        let lints = Lints::new(vec![lint1, lint2]).unwrap();
        assert_eq!(lints.len(), 2);
    }
}
