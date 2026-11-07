pub mod erased_rule_factory;
pub mod inventory;
pub mod rule_factory_registry;

pub use erased_rule_factory::ErasedRuleFactory;
pub use inventory::{RuleFactoryInventory, get_all_rule_factories};
pub use rule_factory_registry::RuleFactoryRegistry;

use serde::de::DeserializeOwned;

/// A factory trait responsible for dynamically creating [`Rule`] instances
/// from deserialized configuration data.
///
/// # Purpose
///
/// In systems where rules are configured via external files (e.g., JSON, YAML, TOML),
/// the application often does not know the concrete types of the rules at compile time.
/// `RuleFabric` acts as a bridge: it holds the metadata required to identify a rule,
/// and provides a method to instantiate the concrete rule from its specific configuration format.
///
/// # Architecture
///
/// This trait is designed to work seamlessly with:
/// - [`RuleFabricRegistry`]: To store and look up factories by name.
/// - `erased_serde`: To perform type-erased, streaming deserialization without intermediate buffering.
///
/// # Associated Types
///
/// - `Rule`: The concrete rule type that this fabric produces. It must implement the [`Rule`] trait.
/// - `Data`: The intermediate configuration structure that can be deserialized from the input format.
///   *Note:* This can be the same type as `Rule` if the rule struct itself serves as the configuration model,
///   but separating them is recommended for complex rules requiring validation before instantiation.
/// - `Error`: The error type returned if the configuration data is invalid or rule creation fails.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
/// # use obsidian_tidy_core::rule::{Category, Rule, RuleMetadata, RuleRunner, RuleFabric, Content, Violation};
/// # use obsidian_tidy_core::Note;
///
/// // 1. Define the configuration structure for the rule
/// #[derive(Debug, Deserialize)]
/// pub struct HeadingSpacingConfig {
///     pub min_spaces: usize,
/// }
///
/// // 2. Define the concrete rule
/// #[derive(Debug)]
/// pub struct HeadingSpacingRule {
///     min_spaces: usize,
/// }
///
/// impl RuleMetadata for HeadingSpacingRule {
///     fn name(&self) -> &str { "heading-spacing" }
///     fn description(&self) -> &str { "Ensures consistent spacing after headings." }
///     fn category(&self) -> Category { Category::Spacing }
/// }
///
/// impl RuleRunner for HeadingSpacingRule {
///     type Error = std::convert::Infallible;
///     
///     fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
///         Ok(vec![]) // Implementation omitted for brevity
///     }
/// }
///
/// // 3. Define the Fabric
/// #[derive(Debug, Default)]
/// pub struct HeadingSpacingFabric;
///
/// impl RuleFabric for HeadingSpacingFabric {
///     type Rule = HeadingSpacingRule;
///     type Data = HeadingSpacingConfig;
///     type Error = std::convert::Infallible;
///
///     fn name_rule(&self) -> &str { "heading-spacing" }
///     fn description_rule(&self) -> &str { "Ensures consistent spacing after headings." }
///     fn category_rule(&self) -> Category { Category::Spacing }
///
///     fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
///         // Validate config here if needed (e.g., min_spaces > 0)
///         if data.min_spaces == 0 {
///             // In a real scenario, return a custom Error variant here
///             panic!("min_spaces must be greater than 0");
///         }
///         
///         Ok(HeadingSpacingRule {
///             min_spaces: data.min_spaces,
///         })
///     }
/// }
/// ```
///
/// # Best Practices
///
/// 1. **Validation:** Use the `create_rule` method to validate the `Data`. If the configuration
///    is semantically invalid (e.g., negative values where only positive are allowed), return a descriptive `Error`.
/// 2. **Statelessness:** Fabrics should ideally be stateless (e.g., unit structs or containing only static metadata).
///    This allows them to be cheaply cloned and stored in a [`RuleFabricRegistry`].
/// 3. **Metadata Consistency:** Ensure that the metadata returned by `name_rule()`, `description_rule()`,
///    and `category_rule()` exactly matches what the resulting `Rule` will report via [`RuleMetadata`].
///
/// [`RuleMetadata`]: crate::rule::RuleMetadata
/// [`Rule`]: crate::rule::Rule
pub trait RuleFactory {
    type Rule: super::Rule;
    type Data: DeserializeOwned;
    type Error: std::error::Error;

    fn id(&self) -> &str;

    fn create_by_serde(&self, data: Self::Data) -> Result<Self::Rule, Self::Error>;

    fn create_default(&self) -> Option<Self::Rule>;
}

macro_rules! impl_rule_factory_with_serde {
    ($item:ident, $rule:ident, $id:literal) => {
        impl $crate::rule::factory::RuleFactory for $item {
            type Rule = $rule;
            type Data = $rule;
            type Error = std::convert::Infallible;

            fn id(&self) -> &str {
                $id
            }

            fn create_by_serde(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
                Ok(data)
            }

            fn create_default(&self) -> Option<Self::Rule> {
                None
            }
        }
    };
}

pub(crate) use impl_rule_factory_with_serde;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, RuleMetadata},
        test_utils::{TestRule, TestRuleFactory},
    };

    #[test]
    fn test_rule_fabric() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let fabric = TestRuleFactory::new(TEST_NAME);
        let data = serde_json::from_str(&json).unwrap();
        let deserialized_rule = RuleFactory::create_by_serde(&fabric, data).unwrap();

        assert_eq!(test_rule, deserialized_rule);
        assert_eq!(test_rule.name(), deserialized_rule.name());
        assert_eq!(test_rule.description(), deserialized_rule.description());
        assert_eq!(test_rule.category(), deserialized_rule.category());
    }
}
