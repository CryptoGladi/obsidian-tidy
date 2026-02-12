pub mod serde;

use super::{Lint, ToggleableLint};
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
pub struct Lints<L: Lint>(Vec<ToggleableLint<L>>);

impl<L> Lints<L>
where
    L: Lint,
{
    #[instrument(skip(lints), err)]
    fn check_unique_name(lints: &[ToggleableLint<L>]) -> Result<(), Error> {
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

    pub fn new(lints: Vec<ToggleableLint<L>>) -> Result<Self, Error> {
        Self::check_unique_name(&lints)?;
        Ok(Self(lints))
    }

    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&ToggleableLint<L>> {
        self.0.iter().find(|lint| lint.name() == name.as_ref())
    }

    pub fn get_mut_by_name(&mut self, name: impl AsRef<str>) -> Option<&mut ToggleableLint<L>> {
        self.0.iter_mut().find(|lint| lint.name() == name.as_ref())
    }
}

impl<T, L> Index<T> for Lints<L>
where
    T: AsRef<str>,
    L: Lint,
{
    type Output = ToggleableLint<L>;

    fn index(&self, index: T) -> &Self::Output {
        self.get_by_name(index).expect("Not found lint by name")
    }
}

impl<T, L> IndexMut<T> for Lints<L>
where
    T: AsRef<str>,
    L: Lint,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_mut_by_name(index).expect("Not found lint by name")
    }
}

impl<L> Deref for Lints<L>
where
    L: Lint,
{
    type Target = Vec<ToggleableLint<L>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lint::Category, test_utils::TestLint};
    use std::sync::Arc;

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";
        let lint1 = ToggleableLint::new(Arc::new(TestLint::new(name, "", Category::Custom)), true);
        let lint2 = ToggleableLint::new(Arc::new(TestLint::new(name, "", Category::Custom)), true);

        let lints = Lints::new(vec![lint1, lint2]);

        assert_eq!(lints.err(), Some(Error::DuplicateName(name.to_string())))
    }

    #[test]
    fn new() {
        let lint1 = ToggleableLint::new(
            Arc::new(TestLint::new("Lint1", "", Category::Content)),
            true,
        );

        let lint2 = ToggleableLint::new(
            Arc::new(TestLint::new("Lint2", "", Category::Content)),
            true,
        );

        let lints = Lints::new(vec![lint1, lint2]).unwrap();
        assert_eq!(lints.len(), 2);
    }
}
