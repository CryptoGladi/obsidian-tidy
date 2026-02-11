pub mod serde;

use super::{Error, ToggleableLint};
use std::collections::HashSet;
use std::ops::{Deref, Index, IndexMut};
use tracing::{instrument, trace};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Lints(Vec<ToggleableLint>);

impl Lints {
    #[instrument(err)]
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

    pub fn get_by_name(&self, name: impl AsRef<str>) -> Option<&ToggleableLint> {
        self.0.iter().find(|lint| lint.name() == name.as_ref())
    }

    pub fn get_mut_by_name(&mut self, name: impl AsRef<str>) -> Option<&mut ToggleableLint> {
        self.0.iter_mut().find(|lint| lint.name() == name.as_ref())
    }
}

impl<T> Index<T> for Lints
where
    T: AsRef<str>,
{
    type Output = ToggleableLint;

    fn index(&self, index: T) -> &Self::Output {
        self.get_by_name(index).expect("Not found lint by name")
    }
}

impl<T> IndexMut<T> for Lints
where
    T: AsRef<str>,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_mut_by_name(index).expect("Not found lint by name")
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
    use crate::{lint::Category, test_utils::TestLint};
    use std::sync::Arc;

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";
        let lint1 = ToggleableLint::new(Arc::new(TestLint::new(name, Category::Custom)), true);
        let lint2 = ToggleableLint::new(Arc::new(TestLint::new(name, Category::Custom)), true);

        let lints = Lints::new(vec![lint1, lint2]);

        assert_eq!(lints.err(), Some(Error::DuplicateName(name.to_string())))
    }

    #[test]
    fn new() {
        let lint1 = ToggleableLint::new(Arc::new(TestLint::new("Lint1", Category::Content)), true);
        let lint2 = ToggleableLint::new(Arc::new(TestLint::new("Lint2", Category::Content)), true);

        let lints = Lints::new(vec![lint1, lint2]).unwrap();
        assert_eq!(lints.len(), 2);
    }
}
