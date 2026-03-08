use super::ErasedRuleFabric;
use crate::rule::Category;
use std::{collections::HashMap, fmt::Debug};

#[derive(Default)]
pub struct RuleFabricRegistry(HashMap<String, Box<dyn ErasedRuleFabric>>);

impl RuleFabricRegistry {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use = "To check for factory collisions for rules"]
    pub fn add(&mut self, fabric: Box<dyn ErasedRuleFabric>) -> Option<Box<dyn ErasedRuleFabric>> {
        self.0.insert(fabric.name_rule().to_string(), fabric)
    }

    pub fn add_unique(&mut self, fabric: Box<dyn ErasedRuleFabric>) {
        let name = fabric.name_rule().to_string();

        if let Some(prev) = self.0.insert(name.clone(), fabric) {
            panic!("Fabric with name '{}' already exists", prev.name_rule());
        }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn ErasedRuleFabric>> {
        self.0.get(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }
}

impl<S> std::ops::Index<S> for RuleFabricRegistry
where
    S: AsRef<str>,
{
    type Output = Box<dyn ErasedRuleFabric>;

    fn index(&self, name: S) -> &Self::Output {
        self.get(name.as_ref()).expect("Fabric not found")
    }
}

impl Debug for RuleFabricRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unused)]
        struct InnerDebugFabric<'a> {
            name_rule: &'a str,
            description_rule: &'a str,
            category_rule: Category,
        }

        impl Debug for InnerDebugFabric<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(self.name_rule)
                    .field("description_rule", &self.description_rule)
                    .field("category_rule", &self.category_rule)
                    .finish()
            }
        }

        f.debug_list()
            .entries(self.0.iter().map(|(name, fabric)| {
                debug_assert_eq!(name, fabric.name_rule());

                InnerDebugFabric {
                    name_rule: fabric.name_rule(),
                    description_rule: fabric.description_rule(),
                    category_rule: fabric.category_rule(),
                }
            }))
            .finish()
    }
}

impl PartialEq for RuleFabricRegistry {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        self.0.iter().all(|(key, self_fabric)| {
            other.0.get(key).map_or(false, |other_fabric| {
                self_fabric.name_rule() == other_fabric.name_rule()
                    && self_fabric.description_rule() == other_fabric.description_rule()
                    && self_fabric.category_rule() == other_fabric.category_rule()
            })
        })
    }
}

impl Eq for RuleFabricRegistry {}

#[macro_export]
macro_rules! rule_fabric_registry {
    [] => {
        $crate::rule::RuleFabricRegistry::new()
    };

    [$fabric:expr] => {{
        let mut registry = $crate::rule::RuleFabricRegistry::new();
        registry.add_unique(Box::new($fabric));
        registry
    }};

    [$($fabric:expr),+ $(,)?] => {{
        let mut registry = $crate::rule::RuleFabricRegistry::new();
        $(
            registry.add_unique(Box::new($fabric));
        )+
        registry
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestRuleFabric;

    #[test]
    fn add() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn double_add() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let collision = registry.add(Box::new(fabric)).is_some();
        assert!(collision)
    }

    #[test]
    #[should_panic]
    fn add_unique() {
        let mut registry = RuleFabricRegistry::new();
        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);

        registry.add_unique(Box::new(fabric.clone()));
        registry.add_unique(Box::new(fabric));
    }

    #[test]
    fn get() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let getted_fabric = registry.get("test-rule").unwrap();

        assert_eq!(getted_fabric.name_rule(), fabric.name_rule());
        assert_eq!(getted_fabric.description_rule(), fabric.description_rule());
        assert_eq!(getted_fabric.category_rule(), fabric.category_rule());
    }

    #[test]
    fn index() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let getted_fabric = &registry["test-rule"];

        assert_eq!(getted_fabric.name_rule(), fabric.name_rule());
        assert_eq!(getted_fabric.description_rule(), fabric.description_rule());
        assert_eq!(getted_fabric.category_rule(), fabric.category_rule());
    }

    #[test]
    fn not_found_get() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);

        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        assert!(registry.get("not-found").is_none());
    }

    #[test]
    fn debug() {
        let mut registry = RuleFabricRegistry::new();
        assert_eq!(format!("{registry:?}"), "[]");

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        assert_eq!(
            format!("{registry:?}"),
            r#"[test-rule { description_rule: "", category_rule: Heading }]"#
        );
    }

    #[test]
    fn len() {
        let registry = RuleFabricRegistry::new();

        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn is_empty() {
        let registry = RuleFabricRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn eq_is_empty() {
        let registry1 = RuleFabricRegistry::new();
        let registry2 = RuleFabricRegistry::new();

        assert_eq!(registry1, registry2);
        assert_eq!(registry1, registry1);
        assert_eq!(registry2, registry2);
    }

    #[test]
    fn eq() {
        let mut registry1 = RuleFabricRegistry::new();
        let mut registry2 = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        registry1.add_unique(Box::new(fabric.clone()));
        registry2.add_unique(Box::new(fabric));

        assert_eq!(registry1, registry2);
    }

    #[test]
    fn not_eq_by_len() {
        let registry1 = RuleFabricRegistry::new();
        let mut registry2 = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        registry2.add_unique(Box::new(fabric));

        assert_ne!(registry1, registry2);
    }

    #[test]
    fn not_eq_by_name() {
        let mut registry1 = RuleFabricRegistry::new();
        let mut registry2 = RuleFabricRegistry::new();

        let fabric1 = TestRuleFabric::new("test-rule1", "", Category::Heading);
        let fabric2 = TestRuleFabric::new("test-rule2", "", Category::Heading);
        registry1.add_unique(Box::new(fabric1));
        registry2.add_unique(Box::new(fabric2));

        assert_ne!(registry1, registry2);
    }

    #[test]
    fn names() {
        let mut registry = RuleFabricRegistry::new();

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        let collision = registry.add(Box::new(fabric.clone())).is_some();
        assert!(!collision);

        let names: Vec<_> = registry.names().collect();
        assert_eq!(names.as_slice(), ["test-rule"])
    }

    #[test]
    fn macro_check() {
        let _empty_registry = rule_fabric_registry![];

        let fabric = TestRuleFabric::new("test-rule", "", Category::Heading);
        let _registry = rule_fabric_registry![fabric];
    }

    #[test]
    #[should_panic]
    fn collision_macro_check() {
        let _empty_registry = rule_fabric_registry![];

        let fabric1 = TestRuleFabric::new("test-rule", "", Category::Heading);
        let fabric2 = fabric1.clone();
        let _registry = rule_fabric_registry![fabric1, fabric2];
    }
}
