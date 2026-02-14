pub mod serde;

use super::{Rule, ToggleableRule};
use std::collections::HashSet;
use std::ops::{Deref, Index, IndexMut};
use thiserror::Error;
use tracing::{instrument, trace};

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Found duplicate name: {0}")]
    DuplicateName(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Rules<R: Rule>(Vec<ToggleableRule<R>>);

impl<R> Rules<R>
where
    R: Rule,
{
    #[instrument(skip(rules), err)]
    fn check_unique_name(rules: &[ToggleableRule<R>]) -> Result<(), Error> {
        trace!("Check unique name");

        let mut names = HashSet::with_capacity(rules.len());
        let iter = rules.iter().map(|rule| rule.name());

        for name in iter {
            if !names.insert(name) {
                return Err(Error::DuplicateName(name.to_string()));
            }
        }

        Ok(())
    }

    pub fn new(rule: Vec<ToggleableRule<R>>) -> Result<Self, Error> {
        Self::check_unique_name(&rule)?;
        Ok(Self(rule))
    }

    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&ToggleableRule<R>> {
        self.0.iter().find(|rule| rule.name() == name.as_ref())
    }

    pub fn get_mut_by_name(&mut self, name: impl AsRef<str>) -> Option<&mut ToggleableRule<R>> {
        self.0.iter_mut().find(|rule| rule.name() == name.as_ref())
    }
}

impl<T, R> Index<T> for Rules<R>
where
    T: AsRef<str>,
    R: Rule,
{
    type Output = ToggleableRule<R>;

    fn index(&self, index: T) -> &Self::Output {
        self.get_by_name(index).expect("Not found rule by name")
    }
}

impl<T, R> IndexMut<T> for Rules<R>
where
    T: AsRef<str>,
    R: Rule,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_mut_by_name(index).expect("Not found rule by name")
    }
}

impl<R> Deref for Rules<R>
where
    R: Rule,
{
    type Target = Vec<ToggleableRule<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rule::Category, test_utils::TestRule};
    use std::sync::Arc;

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";
        let rule1 =
            ToggleableRule::new(Arc::new(TestRule::new(name, "", Category::Other, [])), true);
        let rule2 =
            ToggleableRule::new(Arc::new(TestRule::new(name, "", Category::Other, [])), true);

        let rules = Rules::new(vec![rule1, rule2]);

        assert_eq!(rules.err(), Some(Error::DuplicateName(name.to_string())))
    }

    #[test]
    fn new() {
        let rule1 = ToggleableRule::new(
            Arc::new(TestRule::new("Rule1", "", Category::Content, [])),
            true,
        );

        let rule2 = ToggleableRule::new(
            Arc::new(TestRule::new("Rule2", "", Category::Content, [])),
            true,
        );

        let rules = Rules::new(vec![rule1, rule2]).unwrap();
        assert_eq!(rules.len(), 2);
    }
}
