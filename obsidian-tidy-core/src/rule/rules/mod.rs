// pub mod serde;

use super::{Rule, ToggleableRule};
use crate::rule::erased_rule::{ErasedRule, GetErasedRule};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, Index, IndexMut};
use tracing::{instrument, trace};

type Value = ToggleableRule<Box<dyn ErasedRule>>;

#[derive(Default)]
pub struct Rules(HashMap<String, Value>);

impl IntoIterator for Rules {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Value> for Rules {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let mut rules = Self::new();

        for rule in iter {
            if let Some(duplicate) = rules.add(rule) {
                panic!("Duplicate rule `{}` detected", duplicate.name());
            }
        }

        rules
    }
}

impl Rules {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[instrument(skip_all)]
    pub fn add<R: Rule>(&mut self, rule: ToggleableRule<R>) -> Option<Value> {
        trace!("Add rule `{}`", rule.name());
        let name = rule.name().to_string();
        let is_enabled = rule.is_enabled();
        let erased_rule = rule.into_rule().into_erased();

        self.0.insert(name, rule);
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.0.get(name.as_ref())
    }

    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Value> {
        self.0.get_mut(name.as_ref())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn rules(&self) -> impl Iterator<Item = &Value> {
        self.0.values()
    }
}

impl<T> Index<T> for Rules
where
    T: AsRef<str>,
{
    type Output = Value;

    fn index(&self, index: T) -> &Self::Output {
        self.get(index).expect("Not found rule by name")
    }
}

impl<T> IndexMut<T> for Rules
where
    T: AsRef<str>,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_mut(index).expect("Not found rule by name")
    }
}

#[cfg(test)]
#[allow(clippy::similar_names)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, ErasedRule},
        test_utils::TestRule,
    };
    use std::sync::Arc;

    #[test]
    fn new() {
        let rules = Rules::new();
        assert_eq!(rules.len(), 0);
    }

    /*

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";

        let rule1 =
            ToggleableRule::new(Arc::new(TestRule::new(name, "", Category::Other, [])), true);

        let rule2 = ToggleableRule::new(
            Arc::new(TestRule::new(name, "", Category::Heading, [])),
            true,
        );

        let rules = Rules::new(vec![rule1, rule2]);

        assert_eq!(rules.err(), Some(Error::DuplicateName(name.to_string())));
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
    */
}
