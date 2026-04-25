pub mod serde;

use super::ToggleableRule;
use crate::rule::erased_rule::ErasedRule;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use tracing::{instrument, trace};

type Value = ToggleableRule<Box<dyn ErasedRule>>;

/// A collection of named, toggleable rules stored in a hash map.
///
/// `Rules` provides a convenient way to manage a set of [`ToggleableRule`] instances,
/// each identified by a unique string name (retrieved via [`RuleMetadata::name`]).
/// It supports adding, retrieving, and mutating rules, as well as iterating over
/// the collection.
///
/// # Uniqueness
///
/// Each rule must have a unique name. Attempting to add a rule with a name that
/// already exists will return the previously stored rule as an `Option<Value>`.
///
/// # Examples
///
/// ## Creating an empty collection
///
/// ```
/// use obsidian_tidy_core::rule::Rules;
///
/// let rules = Rules::new();
/// assert!(rules.is_empty());
/// ```
///
/// ## Adding and retrieving rules
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Rules};
/// use obsidian_tidy_core::rule::ToggleableRule;
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule = TestRule::new("my-rule", "My test rule", Category::Content, []);
/// let toggleable = ToggleableRule::new(rule, true);
///
/// let mut rules = Rules::new();
/// assert!(rules.add(toggleable.into()).is_none());
/// assert_eq!(rules.len(), 1);
///
/// let retrieved = rules.get("my-rule").unwrap();
/// assert_eq!(retrieved.name(), "my-rule");
/// assert!(retrieved.is_enabled());
/// ```
///
/// ## Using index access
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Rules};
/// use obsidian_tidy_core::rule::ToggleableRule;
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule = TestRule::new("fmt-rule", "Formatting rule", Category::Heading, []);
/// let toggleable = ToggleableRule::new(rule, true);
///
/// let mut rules = Rules::new();
/// rules.add(toggleable.into());
///
/// // Immutable index
/// assert_eq!(rules["fmt-rule"].name(), "fmt-rule");
///
/// // Mutable index
/// rules["fmt-rule"].disable();
/// assert!(rules["fmt-rule"].is_disabled());
/// ```
///
/// ## Building from an iterator
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Rules};
/// use obsidian_tidy_core::rule::ToggleableRule;
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let rule1 = TestRule::new("rule-a", "Rule A", Category::Heading, []);
/// let rule2 = TestRule::new("rule-b", "Rule B", Category::Other, []);
///
/// let rules = Rules::try_from_iter([
///     ToggleableRule::new(rule1, true).into_erased(),
///     ToggleableRule::new(rule2, false).into_erased(),
/// ]);
///
/// assert!(rules.is_ok());
/// assert_eq!(rules.unwrap().len(), 2);
/// ```
///
/// [`RuleMetadata::name`]: crate::rule::RuleMetadata::name
#[derive(Default, Debug)]
pub struct Rules(HashMap<String, Value>);

impl IntoIterator for Rules {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    /// Consumes the collection and returns an iterator over the rules.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule1 = TestRule::new("alpha", "Alpha rule", Category::Heading, []);
    /// let rule2 = TestRule::new("beta", "Beta rule", Category::Other, []);
    ///
    /// let mut rules = Rules::new();
    /// rules.add(ToggleableRule::new(rule1, true).into_erased());
    /// rules.add(ToggleableRule::new(rule2, false).into_erased());
    ///
    /// let mut names: Vec<_> = rules.into_iter().map(|(name, _)| name).collect();
    /// names.sort();
    /// assert_eq!(names, vec!["alpha".to_string(), "beta".to_string()]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Rules {
    /// Creates a new, empty `Rules` collection.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::Rules;
    ///
    /// let rules = Rules::new();
    /// assert!(rules.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Create a new, empty `Rules` collection, bit with capacity
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::Rules;
    ///
    /// let rules = Rules::with_capacity(100);
    /// assert!(rules.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Adds a rule to the collection, keyed by its name.
    ///
    /// If a rule with the same name already exists, it is replaced and the
    /// previous rule is returned as `Some(Value)`. Otherwise, returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    ///
    /// // Adding a new rule returns None
    /// let rule = TestRule::new("first", "First rule", Category::Content, []);
    /// assert!(rules.add(ToggleableRule::new(rule, true).into_erased()).is_none());
    ///
    /// // Adding a duplicate returns the previous rule
    /// let duplicate = TestRule::new("first", "Duplicate", Category::Other, []);
    /// let old = rules.add(ToggleableRule::new(duplicate, false).into_erased());
    /// assert!(old.is_some());
    /// assert_eq!(old.unwrap().name(), "first");
    /// ```
    #[instrument(skip_all)]
    #[must_use]
    pub fn add(&mut self, rule: Value) -> Option<Value> {
        trace!("Add rule `{}`", rule.name());

        self.0.insert(rule.name().to_string(), rule)
    }

    /// Returns a reference to the rule with the given name.
    ///
    /// Returns `None` if no rule with the given name exists.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// let rule = TestRule::new("lookup", "Lookup rule", Category::Heading, []);
    /// rules.add(ToggleableRule::new(rule, true).into_erased());
    ///
    /// assert!(rules.get("lookup").is_some());
    /// assert!(rules.get("nonexistent").is_none());
    /// ```
    pub fn get(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.0.get(name.as_ref())
    }

    /// Returns a mutable reference to the rule with the given name.
    ///
    /// Returns `None` if no rule with the given name exists.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// let rule = TestRule::new("toggle", "Toggle rule", Category::Other, []);
    /// rules.add(ToggleableRule::new(rule, true).into_erased());
    ///
    /// // Mutate the rule through the mutable reference
    /// rules.get_mut("toggle").unwrap().disable();
    /// assert!(rules.get("toggle").unwrap().is_disabled());
    ///
    /// assert!(rules.get_mut("nonexistent").is_none());
    /// ```
    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut Value> {
        self.0.get_mut(name.as_ref())
    }

    /// Returns the number of rules in the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// assert_eq!(rules.len(), 0);
    ///
    /// let rule = TestRule::new("count", "Count rule", Category::Content, []);
    /// rules.add(ToggleableRule::new(rule, true).into_erased());
    /// assert_eq!(rules.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collection contains no rules.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// assert!(rules.is_empty());
    ///
    /// let rule = TestRule::new("empty", "Empty rule", Category::Heading, []);
    /// rules.add(ToggleableRule::new(rule, true).into_erased());
    /// assert!(!rules.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over all rules with their names.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// let rule = TestRule::new("iter-with-name", "Named rule", Category::Heading, []);
    /// rules.add(ToggleableRule::new(rule, true).into_erased());
    ///
    /// for (name, rule) in rules.iter() {
    ///     assert_eq!(name, rule.name());
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.0.iter()
    }

    /// Returns an iterator over the names of all rules in the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// let rule1 = TestRule::new("x", "X rule", Category::Heading, []);
    /// let rule2 = TestRule::new("y", "Y rule", Category::Other, []);
    /// rules.add(ToggleableRule::new(rule1, true).into_erased());
    /// rules.add(ToggleableRule::new(rule2, true).into_erased());
    ///
    /// let mut names: Vec<_> = rules.names().collect();
    /// names.sort();
    /// assert_eq!(names, vec!["x", "y"]);
    /// ```
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Returns an iterator over all rules in the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let mut rules = Rules::new();
    /// let rule1 = TestRule::new("iter-a", "A", Category::Heading, []);
    /// let rule2 = TestRule::new("iter-b", "B", Category::Other, []);
    /// rules.add(ToggleableRule::new(rule1, true).into_erased());
    /// rules.add(ToggleableRule::new(rule2, false).into_erased());
    ///
    /// let count = rules.rules().count();
    /// assert_eq!(count, 2);
    /// ```
    pub fn rules(&self) -> impl Iterator<Item = &Value> {
        self.0.values()
    }

    /// Creates a `Rules` collection from an iterator of rules.
    ///
    /// Returns `Err(Value)` if a duplicate rule name is encountered,
    /// containing the first rule that was already inserted with that name.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule1 = TestRule::new("unique-1", "Unique one", Category::Heading, []);
    /// let rule2 = TestRule::new("unique-2", "Unique two", Category::Other, []);
    ///
    /// let result = Rules::try_from_iter([
    ///     ToggleableRule::new(rule1, true).into_erased(),
    ///     ToggleableRule::new(rule2, false).into_erased(),
    /// ]);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap().len(), 2);
    /// ```
    ///
    /// Duplicate names cause an error to be returned:
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, Rules};
    /// use obsidian_tidy_core::rule::ToggleableRule;
    /// use obsidian_tidy_core::test_utils::TestRule;
    ///
    /// let rule1 = TestRule::new("dup", "First", Category::Heading, []);
    /// let rule2 = TestRule::new("dup", "Second", Category::Other, []);
    ///
    /// let result = Rules::try_from_iter([
    ///     ToggleableRule::new(rule1.clone(), true).into_erased(),
    ///     ToggleableRule::new(rule2, false).into_erased(),
    /// ]);
    ///
    /// assert!(result.is_err());
    /// assert_eq!(result.err().unwrap().name(), "dup");
    /// ```
    #[instrument(skip_all)]
    pub fn try_from_iter<I>(iter: I) -> Result<Self, Value>
    where
        I: IntoIterator<Item = Value>,
    {
        trace!("run try_from_iter for Rules");
        let mut rules = Self::new();

        for rule in iter {
            if let Some(duplicate) = rules.add(rule) {
                return Err(duplicate);
            }
        }

        Ok(rules)
    }
}

/// Enables immutable indexing access by rule name using `rules["name"]`.
///
/// # Panics
///
/// Panics if no rule with the given name exists. Use [`Rules::get`] for
/// a non-panicking alternative.
///
/// # Example
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Rules};
/// use obsidian_tidy_core::rule::ToggleableRule;
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let mut rules = Rules::new();
/// let rule = TestRule::new("idx", "Indexable rule", Category::Heading, []);
/// rules.add(ToggleableRule::new(rule, true).into_erased());
///
/// assert_eq!(rules["idx"].name(), "idx");
/// assert_eq!(rules["idx"].description(), "Indexable rule");
/// ```
impl<T> Index<T> for Rules
where
    T: AsRef<str>,
{
    type Output = Value;

    fn index(&self, index: T) -> &Self::Output {
        self.get(index).expect("Not found rule by name")
    }
}

/// Enables mutable indexing access by rule name using `rules["name"]`.
///
/// # Panics
///
/// Panics if no rule with the given name exists. Use [`Rules::get_mut`] for
/// a non-panicking alternative.
///
/// # Example
///
/// ```
/// use obsidian_tidy_core::rule::{Category, Rules};
/// use obsidian_tidy_core::rule::ToggleableRule;
/// use obsidian_tidy_core::test_utils::TestRule;
///
/// let mut rules = Rules::new();
/// let rule = TestRule::new("mut-idx", "Mutable indexable rule", Category::Other, []);
/// rules.add(ToggleableRule::new(rule, true).into_erased());
///
/// rules["mut-idx"].disable();
/// assert!(rules["mut-idx"].is_disabled());
/// ```
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
        rule::{Category, RuleMetadata},
        test_utils::TestRule,
    };
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        let rules = Rules::new();
        assert_eq!(rules.len(), 0);
    }

    #[test]
    #[traced_test]
    fn default() {
        let rules = Rules::default();
        assert_eq!(rules.len(), 0);
    }

    #[test]
    #[traced_test]
    fn is_empty() {
        let rules = Rules::default();
        assert!(rules.is_empty());
    }

    #[test]
    #[traced_test]
    fn add() {
        let rule1 = TestRule::new("rule1", "Rule1", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let rule2 = TestRule::new("rule2", "Rule2", Category::Other, []);
        let rule2 = ToggleableRule::new(rule2, false);

        let mut rules = Rules::new();
        let check = rules.add(rule1.into()).is_none();
        assert!(check);

        let check = rules.add(rule2.into()).is_none();
        assert!(check);

        assert_eq!(rules.len(), 2);
    }

    #[test]
    #[traced_test]
    fn add_with_duplicate() {
        let name = "DuplicateName";

        let rule1 = TestRule::new(name, "Rule1", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let rule2 = TestRule::new(name, "Rule2", Category::Other, []);
        let rule2 = ToggleableRule::new(rule2, false);

        let mut rules = Rules::new();
        let check = rules.add(rule1.into()).is_none();
        assert!(check);

        let result = rules.add(rule2.into());
        assert!(result.is_some());
    }

    #[test]
    #[traced_test]
    fn get() {
        let rule1 = TestRule::new("testrule", "", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let mut rules = Rules::new();
        let check = rules.add(rule1.clone().into()).is_none();
        assert!(check);

        let gotten_rule = rules.get("testrule").unwrap();
        assert_eq!(gotten_rule.name(), rule1.name());
        assert_eq!(gotten_rule.description(), rule1.description());
        assert_eq!(gotten_rule.category(), rule1.category());
        assert_eq!(gotten_rule.is_enabled(), rule1.is_enabled());
    }

    #[test]
    #[traced_test]
    fn not_found_get() {
        let rules = Rules::new();
        assert!(rules.get("name").is_none());
    }

    #[test]
    #[traced_test]
    fn mut_get() {
        let rule1 = TestRule::new("testrule", "", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let mut rules = Rules::new();
        let check = rules.add(rule1.into()).is_none();
        assert!(check);

        rules.get_mut("testrule").unwrap().disable();
        assert!(rules.get("testrule").unwrap().is_disabled());
    }

    #[test]
    #[traced_test]
    fn not_found_get_mut() {
        let mut rules = Rules::new();
        assert!(rules.get_mut("name").is_none());
    }

    #[test]
    #[traced_test]
    fn index() {
        let rule1 = TestRule::new("testrule", "", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let mut rules = Rules::new();
        let check = rules.add(rule1.clone().into()).is_none();
        assert!(check);

        let gotten_rule = &rules["testrule"];
        assert_eq!(gotten_rule.name(), rule1.name());
        assert_eq!(gotten_rule.description(), rule1.description());
        assert_eq!(gotten_rule.category(), rule1.category());
        assert_eq!(gotten_rule.is_enabled(), rule1.is_enabled());
    }

    #[test]
    #[traced_test]
    fn mut_index() {
        let rule1 = TestRule::new("testrule", "", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let mut rules = Rules::new();
        let check = rules.add(rule1.clone().into()).is_none();
        assert!(check);

        rules["testrule"].disable();
        assert!(rules["testrule"].is_disabled());
    }

    #[test]
    #[traced_test]
    fn try_from_iter() {
        let rule1 = TestRule::new("rule1", "Rule one", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let rule2 = TestRule::new("rule2", "Rule two", Category::Other, []);
        let rule2 = ToggleableRule::new(rule2, false);

        let vec_rules = vec![rule1.into_erased(), rule2.into_erased()];
        let rules = Rules::try_from_iter(vec_rules).unwrap();

        assert_eq!(rules.len(), 2);
    }

    #[test]
    #[traced_test]
    fn try_from_iter_with_duplicate() {
        let rule1 = TestRule::new("rule1", "Rule one", Category::Heading, []);
        let rule1 = ToggleableRule::new(rule1, true);

        let rule2 = TestRule::new("rule1", "Rule two", Category::Other, []);
        let rule2 = ToggleableRule::new(rule2, false);

        let vec_rules = vec![rule1.clone().into_erased(), rule2.into_erased()];
        let rules = Rules::try_from_iter(vec_rules);

        assert_eq!(rules.err().unwrap().description(), rule1.description());
    }
}
