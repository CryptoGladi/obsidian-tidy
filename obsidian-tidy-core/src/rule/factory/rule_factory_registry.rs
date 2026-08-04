//! A registry for managing rule factories ([`ErasedRuleFabric`]).
//!
//! This module provides a centralized, type-erased collection for storing and retrieving
//! rule factories by name. It enables dynamic rule creation, configuration-based
//! deserialization, and runtime rule registration without requiring compile-time
//! knowledge of all available rules.
//!
//! # Key Features
//! - **Type Erasure:** Stores `Box<dyn ErasedRuleFabric + Send + Sync>`, allowing
//!   heterogeneous rule types in a single collection.
//! - **Collision Safety:** Prevents duplicate registrations with explicit return values.
//! - **Macro Convenience:** The [`crate::rule_fabric_registry`] macro simplifies static registry
//!   creation at compile time.
//! - **Thread-Safe Storage:** Factories are `Send + Sync`, making the registry safe to share
//!   across threads when wrapped in `Arc` or `RwLock`.

use crate::rule::factory::erased_rule_factory::ErasedRuleFactory;
use core::convert::AsRef;
use std::collections::HashMap;

/// A registry that maps rule names to their corresponding factories.
///
/// `RuleFabricRegistry` acts as a lookup table for [`ErasedRuleFabric`] implementations.
/// Each factory is stored under its unique rule name, enabling dynamic instantiation
/// of rules during deserialization or runtime configuration.
///
/// # Thread Safety
///
/// The stored factories are `Send + Sync`, but the registry itself is not thread-safe
/// for mutation. Use `Arc<RwLock<RuleFabricRegistry>>` or similar for concurrent access.
///
/// # Examples
///
/// ```
/// use obsidian_tidy_core::rule::{Category, RuleFabricRegistry};
/// use obsidian_tidy_core::test_utils::TestRuleFabric;
///
/// let mut registry = RuleFabricRegistry::new();
/// let fabric = TestRuleFabric::new("my-rule", "Description", Category::Heading);
///
/// // Add a factory (returns `None` if name is unique)
/// assert!(registry.add(Box::new(fabric)).is_none());
///
/// // Retrieve by name
/// let retrieved = registry.get("my-rule").unwrap();
/// assert_eq!(retrieved.name_rule(), "my-rule");
/// ```
#[derive(Default)]
pub struct RuleFactoryRegistry(HashMap<String, Box<dyn ErasedRuleFactory + Send + Sync>>);

impl RuleFactoryRegistry {
    /// Creates an empty registry
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::RuleFabricRegistry;
    ///
    /// let registry = RuleFabricRegistry::new();
    /// assert!(registry.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates an empty registry with the specified initial capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::RuleFabricRegistry;
    ///
    /// let registry = RuleFabricRegistry::with_capacity(10);
    /// assert!(registry.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of registered rule factories
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the registry contains no rule factories.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::RuleFabricRegistry;
    /// assert!(RuleFabricRegistry::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Registers a new rule factory under its unique name.
    ///
    /// If a factory with the same name already exists, it is replaced and the
    /// previous factory is returned as `Some`. Otherwise, returns `None`.
    ///
    /// # Return Value
    /// - `None`: The factory was successfully registered (name was unique).
    /// - `Some(Box<dyn ErasedRuleFabric>)`: A factory with this name already existed
    ///   and has been replaced.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, RuleFabricRegistry};
    /// use obsidian_tidy_core::test_utils::TestRuleFabric;
    ///
    /// let mut registry = RuleFabricRegistry::new();
    /// let fabric = TestRuleFabric::new("my-rule", "", Category::Heading);
    ///
    /// assert!(registry.add(Box::new(fabric.clone())).is_none());
    /// let replaced = registry.add(Box::new(fabric));
    /// assert!(replaced.is_some());
    /// ```
    #[must_use = "To check for factory collisions for rules"]
    pub fn add(
        &mut self,
        fabric: Box<dyn ErasedRuleFactory + Send + Sync>,
    ) -> Option<Box<dyn ErasedRuleFactory + Send + Sync>> {
        self.0.insert(fabric.name().to_string(), fabric)
    }

    /// Returns a reference to the rule factory with the given name.
    ///
    /// Returns `None` if no factory is registered under the specified name.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, RuleFabricRegistry};
    /// use obsidian_tidy_core::test_utils::TestRuleFabric;
    /// use obsidian_tidy_core::rule_fabric_registry;
    ///
    /// let fabric = TestRuleFabric::new("test", "", Category::Heading);
    /// let registry = rule_fabric_registry![fabric];
    ///
    /// assert!(registry.get("test").is_some());
    /// assert!(registry.get("missing").is_none());
    /// ```
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&(dyn ErasedRuleFactory + Send + Sync)> {
        self.0.get(name).map(Box::as_ref)
    }

    /// Returns an iterator over all registered rule names
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(AsRef::as_ref)
    }

    /// Returns an iterator over all registered rule factories.
    pub fn fabrics(&self) -> impl Iterator<Item = &(dyn ErasedRuleFactory + Send + Sync)> {
        self.0.values().map(AsRef::as_ref)
    }

    /// Returns `true` if the registry contains a rule factory with the given name.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_core::rule::{Category, RuleFabricRegistry};
    /// use obsidian_tidy_core::test_utils::TestRuleFabric;
    /// use obsidian_tidy_core::rule_fabric_registry;
    ///
    /// let fabric = TestRuleFabric::default();
    /// let registry = rule_fabric_registry![fabric];
    ///
    /// assert!(registry.contains("test-rule"));
    /// assert!(!registry.contains("other-rule"));
    /// ```
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}

/// Creates a [`RuleFabricRegistry`] from a list of rule factories.
///
/// This macro provides a concise way to statically initialize a registry.
/// It automatically converts each provided factory using `IntoErasedRuleFabric`
/// and panics if a duplicate rule name is detected.
///
/// # Syntax
/// - `rule_fabric_registry![]` → empty registry
/// - `rule_fabric_registry![fabric]` → registry with one factory
/// - `rule_fabric_registry![fabric1, fabric2, ...]` → registry with multiple factories
///
/// # Requirements
/// Each `$fabric` must implement [`IntoErasedRuleFabric`](crate::rule::rule_fabric::erased_rule_fabric::IntoErasedRuleFabric)
/// (or be convertible to `Box<dyn ErasedRuleFabric + Send + Sync>`).
///
/// # Panics
/// Panics at runtime if two or more factories share the same rule name.
/// This check is performed during macro execution, not at compile time.
///
/// # Examples
///
/// ```
/// use obsidian_tidy_core::rule::Category;
/// use obsidian_tidy_core::test_utils::TestRuleFabric;
/// use obsidian_tidy_core::rule_fabric_registry;
///
/// // Empty registry
/// let empty = rule_fabric_registry![];
/// assert!(empty.is_empty());
///
/// // Single factory
/// let f1 = TestRuleFabric::new("rule-a", "", Category::Heading);
/// let single = rule_fabric_registry![f1.clone()];
/// assert_eq!(single.len(), 1);
///
/// // Multiple factories
/// let f2 = TestRuleFabric::new("rule-b", "", Category::Content);
/// let multi = rule_fabric_registry![f1, f2];
/// assert_eq!(multi.len(), 2);
/// ```
#[macro_export]
macro_rules! rule_fabric_registry {
    [] => {
        $crate::rule::RuleFactoryRegistry::new()
    };

    [$($fabric:expr),+ $(,)?] => {{
        use $crate::rule::factory::erased_rule_factory::IntoErasedRuleFactory;

        let mut registry = $crate::rule::RuleFactoryRegistry::new();
        $(
            if let Some(prev) = registry.add($fabric.into_erased()) {
                panic!("Fabric with name '{}' already exists", prev.name());
            }
        )+
        registry
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::factory::erased_rule_factory::IntoErasedRuleFactory, test_utils::TestRuleFactory,
    };

    mod macro_rule_fabric_registry {
        use super::*;

        #[test]
        fn empty() {
            let registry = rule_fabric_registry![];
            assert_eq!(registry.len(), 0);
        }

        #[test]
        fn one_element() {
            let fabric = TestRuleFactory::new("test-rule");
            let registry = rule_fabric_registry![fabric];

            assert_eq!(registry.len(), 1);
        }

        #[test]
        fn many_elements() {
            let fabric1 = TestRuleFactory::new("test-rule1");
            let fabric2 = TestRuleFactory::new("test-rule2");

            let registry = rule_fabric_registry![fabric1, fabric2];

            assert_eq!(registry.len(), 2);
        }

        #[test]
        #[should_panic]
        fn duplicate() {
            let fabric = TestRuleFactory::new("test-rule");
            let _registry = rule_fabric_registry![fabric.clone(), fabric];
        }
    }

    #[test]
    fn add() {
        let mut registry = RuleFactoryRegistry::new();

        let fabric = TestRuleFactory::new("test-rule");
        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn double_add() {
        let mut registry = RuleFactoryRegistry::new();

        let fabric = TestRuleFactory::new("test-rule");

        let collision = registry.add(fabric.clone().into_erased()).is_some();
        assert!(!collision);

        let collision = registry.add(fabric.into_erased()).is_some();
        assert!(collision)
    }

    #[test]
    fn get() {
        let mut registry = RuleFactoryRegistry::new();

        let fabric = TestRuleFactory::new("test-rule");

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let getted_fabric = registry.get("test-rule").unwrap();

        assert_eq!(getted_fabric.name(), fabric.name());
    }

    #[test]
    fn not_found_get() {
        let mut registry = RuleFactoryRegistry::new();

        let fabric = TestRuleFactory::new("test-rule");

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        assert!(registry.get("not-found").is_none());
    }

    #[test]
    fn len() {
        let registry = RuleFactoryRegistry::new();

        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn is_empty() {
        let registry = RuleFactoryRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn names() {
        let mut registry = RuleFactoryRegistry::new();

        let fabric = TestRuleFactory::new("test-rule");
        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let names: Vec<_> = registry.names().collect();
        assert_eq!(names.as_slice(), ["test-rule"]);
    }

    #[test]
    fn fabrics() {
        let fabric = TestRuleFactory::new("test-rule");
        let registry = rule_fabric_registry![fabric.clone()];

        let fabrices = registry.fabrics();
        let data: Vec<_> = fabrices.map(|fabric| fabric.name()).collect();

        assert_eq!(data.len(), 1);
        assert_eq!(data[0], fabric.name());
    }

    #[test]
    fn contains() {
        let fabric = TestRuleFactory::new("test-rule");
        let registry = rule_fabric_registry![fabric];

        assert!(registry.contains("test-rule"));
        assert!(!registry.contains("other-rule"));
    }

    #[test]
    fn macro_check() {
        let _empty_registry = rule_fabric_registry![];

        let fabric = TestRuleFactory::new("test-rule");
        let _registry = rule_fabric_registry![fabric];
    }

    #[test]
    #[should_panic]
    fn collision_macro_check() {
        let _empty_registry = rule_fabric_registry![];

        let fabric1 = TestRuleFactory::new("test-rule");
        let fabric2 = fabric1.clone();
        let _registry = rule_fabric_registry![fabric1, fabric2];
    }
}
