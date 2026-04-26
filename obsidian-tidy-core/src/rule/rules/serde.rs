//! Serialization and deserialization of [`Rules`] collections using `serde` and `erased-serde`.
//!
//! This module provides a robust, type-safe mechanism for serializing and deserializing heterogeneous
//! rule collections. It avoids runtime type tags (`typetag`) and format-specific buffers (`RawValue`),
//! instead relying on a streaming seed-based architecture combined with a factory registry.
//!
//! # Architecture
//!
//! Deserialization is implemented as a chain of [`serde::de::DeserializeSeed`]s:
//! 1. [`RulesSeed`] reads the top-level map of rule names.
//! 2. For each key, it resolves the corresponding factory from a [`RuleFabricRegistry`].
//! 3. [`ErasedToggleableRuleSeed`] parses the wrapper object, extracts the `enable` flag,
//!    and delegates the `config` field to the factory via [`ErasedRuleSeed`].
//! 4. The factory deserializes the config into a concrete rule type and returns a
//!    `ToggleableRule<Box<dyn ErasedRule>>`.
//!
//! # Data Format
//!
//! The expected format uses a nested structure to cleanly separate lifecycle control from
//! rule-specific configuration:
//!
//! ```json
//! {
//!   "empty-content": {
//!     "enable": true,
//!     "config": {}
//!   },
//!   "heading-spacing": {
//!     "enable": false,
//!     "config": { "min_spaces": 2, "max_spaces": 4 }
//!   }
//! }
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use serde_json;
//! use my_crate::rule::{Rules, RuleFabricRegistry, RulesSeed};
//!
//! // 1. Initialize registry with available rule factories
//! let mut registry = RuleFabricRegistry::new();
//! registry.add_unique(Box::new(EmptyContent::fabric()));
//! registry.add_unique(Box::new(HeadingSpacing::fabric()));
//!
//! // 2. Deserialize configuration
//! let json = r#"{ "empty-content": { "enable": true, "config": {} } }"#;
//! let seed = RulesSeed::new(&registry);
//! let rules = seed.deserialize(&mut serde_json::Deserializer::from_str(json))?;
//!
//! // 3. Serialize back to JSON (directly on the collection)
//! let output = serde_json::to_string_pretty(&rules)?;
//! ```
//!
//! # Extending with New Rules
//!
//! To add a rule that supports this serialization format:
//! 1. Derive `Serialize` and `Deserialize` on your rule/config struct.
//! 2. Implement `RuleConstMetadata` and `RuleRunner`.
//! 3. Provide a factory (e.g., via `GetFabricFromRuleConstMetadata`).
//! 4. Register the factory in the `RuleFabricRegistry` at application startup.
//!
//! No changes to this module are required. The factory will automatically receive the `config`
//! object and deserialize it into your concrete type.
//!
//! # Validation & Errors
//!
//! The deserializer strictly validates input and returns descriptive `serde::de::Error` for:
//! - `Unknown rule` — rule name not found in the registry
//! - `missing field` — absence of `enable` or `config`
//! - `duplicate field` — repeated `enable` or `config` keys
//! - `unknown field` — extra fields in the wrapper object
//! - `Duplicate rule` — same rule name appearing multiple times in the input
//! - Custom errors from `RuleFabric::create_rule` for invalid rule-specific configuration
//!
//! # Performance
//!
//! - **Zero intermediate buffering**: streams directly into the factory via `erased-serde`.
//! - **Key optimization**: uses `&'de str` for map keys to avoid string allocations.
//! - **Capacity hinting**: pre-allocates `Rules` using `MapAccess::size_hint()`.
//! - **Format agnostic**: works with `JSON`, `YAML`, `TOML`, `MessagePack`, or any `serde`-compatible format.

use std::borrow::Cow;

use super::Rules;
use crate::rule::{ErasedRule, ErasedRuleFabric, RuleFabricRegistry, ToggleableRule};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{DeserializeSeed, Error, Visitor},
    ser::SerializeMap,
};
use tracing::instrument;

impl Serialize for Rules {
    #[instrument(skip(serializer))]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        tracing::debug!("serializing `Rules`");

        let mut map = serializer.serialize_map(Some(self.len()))?;

        #[derive(Serialize)]
        struct Wrapper<'a, T> {
            enable: bool,

            config: &'a T,
        }

        for (name, rule) in self.iter() {
            map.serialize_entry(
                name,
                &Wrapper {
                    enable: rule.is_enabled(),
                    config: rule.as_rule(),
                },
            )?;
        }

        map.end()
    }
}

pub struct ErasedRuleSeed<'a> {
    fabric: &'a dyn ErasedRuleFabric,
}

impl<'de> DeserializeSeed<'de> for ErasedRuleSeed<'_> {
    type Value = Box<dyn ErasedRule>;

    #[instrument(skip_all)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        tracing::trace!(
            "deserialize `ErasedRuleFabric` by fabric: {}",
            self.fabric.name_rule()
        );

        let mut erased_deserializer = &mut <dyn erased_serde::Deserializer>::erase(deserializer);

        let rule = self
            .fabric
            .create_rule(&mut erased_deserializer)
            .map_err(D::Error::custom)?;

        Ok(rule)
    }
}

pub struct ErasedToggleableRuleSeed<'a> {
    fabric: &'a dyn ErasedRuleFabric,
}

impl<'de> DeserializeSeed<'de> for ErasedToggleableRuleSeed<'_> {
    type Value = ToggleableRule<Box<dyn ErasedRule>>;

    #[instrument(skip_all)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        tracing::trace!(
            "deserialize `ErasedToggleableRuleSeed` by fabric: {}",
            self.fabric.name_rule()
        );

        #[derive(Deserialize, Debug)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Enable,
            Config,
        }

        impl Field {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    Field::Enable => "enable",
                    Field::Config => "config",
                }
            }
        }

        impl std::fmt::Display for Field {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        const FIELDS: &[&str] = &[Field::Enable.as_str(), Field::Config.as_str()];

        struct WrapperVisitor<'a> {
            fabric: &'a dyn ErasedRuleFabric,
        }

        impl<'de> Visitor<'de> for WrapperVisitor<'_> {
            type Value = ToggleableRule<Box<dyn ErasedRule>>;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "map with fields: `{}`", FIELDS.join("`, `"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut enable = None;
                let mut config = None;

                while let Some(key) = map.next_key::<Field>()? {
                    tracing::trace!("parse key: `{key}`");

                    match key {
                        Field::Enable => {
                            if enable.is_some() {
                                return Err(M::Error::duplicate_field(Field::Enable.as_str()));
                            }
                            enable = Some(map.next_value()?);
                        }
                        Field::Config => {
                            if config.is_some() {
                                return Err(M::Error::duplicate_field(Field::Config.as_str()));
                            }

                            config = Some(map.next_value_seed(ErasedRuleSeed {
                                fabric: self.fabric,
                            })?);
                        }
                    }
                }

                let enable =
                    enable.ok_or_else(|| M::Error::missing_field(Field::Enable.as_str()))?;
                let config =
                    config.ok_or_else(|| M::Error::missing_field(Field::Config.as_str()))?;

                Ok(ToggleableRule::new(config, enable))
            }
        }

        deserializer.deserialize_map(WrapperVisitor {
            fabric: self.fabric,
        })
    }
}

/// A seed for deserializing a [`Rules`] collection using a provided [`RuleFabricRegistry`].
///
/// # Example
/// ```ignore
/// use obsidian_tidy_core::rule::rules::serde::RulesSeed;
/// use serde::de::DeserializeSeed;
///
/// let seed = RulesSeed::new(&registry);
/// let rules = seed.deserialize(&mut serde_json::Deserializer::from_str(json)).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct RulesSeed<'a> {
    registry: &'a RuleFabricRegistry,
}

impl<'a> RulesSeed<'a> {
    #[must_use]
    pub const fn new(registry: &'a RuleFabricRegistry) -> Self {
        Self { registry }
    }
}

impl<'de> DeserializeSeed<'de> for RulesSeed<'_> {
    type Value = Rules;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RulesVisitor<'a> {
            registry: &'a RuleFabricRegistry,
        }

        impl<'de> Visitor<'de> for RulesVisitor<'_> {
            type Value = Rules;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(
                    "map of rule configurations: { rule_name: { enabled: bool, config: {...} } }",
                )
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut rules = Rules::with_capacity(map.size_hint().unwrap_or(0));

                while let Some(rule_name) = map.next_key::<Cow<'de, str>>()? {
                    let fabric = self
                        .registry
                        .get(rule_name.as_ref())
                        .ok_or(M::Error::custom(format!(
                            "Unknown rule '{}'. Available: {:?}",
                            rule_name,
                            self.registry.names().collect::<Vec<_>>()
                        )))?;

                    let rule = map.next_value_seed(ErasedToggleableRuleSeed { fabric })?;

                    if let Some(duplicate_rule) = rules.add(rule) {
                        return Err(M::Error::custom(format!(
                            "Duplicate rule '{}'",
                            duplicate_rule.name()
                        )));
                    }
                }

                Ok(rules)
            }
        }

        let visitor = RulesVisitor {
            registry: self.registry,
        };

        deserializer.deserialize_map(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::Category,
        rule_fabric_registry,
        test_utils::{TestRule, TestRuleFabric},
    };
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn serialize() {
        let rule = TestRule::new("test-rule", "Test rule", Category::Heading, []);
        let rule = ToggleableRule::new(rule, false);

        let mut rules = Rules::new();
        if let Some(duplicate) = rules.add(rule.into_erased()) {
            panic!("Found duplicate: {:?}", duplicate);
        }

        let result = serde_json::to_string(&rules).unwrap();
        assert_eq!(
            result,
            r#"{"test-rule":{"enable":false,"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]}}}"#
        );
    }

    #[test]
    #[traced_test]
    fn deserialize() {
        let fabric = TestRuleFabric::new("test-rule", "Test rule", Category::Heading);
        let registry = rule_fabric_registry![fabric];

        let test_string = r#"{"test-rule":{"enable":false,"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]}}}"#;

        let seed = RulesSeed::new(&registry);
        let rules = seed
            .deserialize(&mut serde_json::Deserializer::from_str(test_string))
            .unwrap();

        let data: Vec<_> = rules
            .rules()
            .map(|rule| {
                (
                    rule.is_enabled(),
                    rule.name(),
                    rule.description(),
                    rule.category(),
                )
            })
            .collect();

        assert_eq!(data, [(false, "test-rule", "Test rule", Category::Heading)]);
    }

    #[test]
    #[traced_test]
    fn serialize_empty_rules() {
        let rules = Rules::new();
        let json = serde_json::to_string(&rules).unwrap();

        assert_eq!(json, "{}");
    }

    #[test]
    #[traced_test]
    fn deserialize_empty_rules() {
        let registry = RuleFabricRegistry::new();
        let seed = RulesSeed::new(&registry);

        let rules = seed
            .deserialize(&mut serde_json::Deserializer::from_str("{}"))
            .unwrap();

        assert!(rules.is_empty());
    }

    #[test]
    #[traced_test]
    fn deserialize_unknown_rule() {
        let registry = RuleFabricRegistry::new();
        let seed = RulesSeed::new(&registry);

        let json = r#"{"unknown":{"enable":true,"config":{}}}"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("Unknown rule 'unknown'"));
    }

    #[test]
    #[traced_test]
    fn deserialize_missing_enable() {
        let fabric = TestRuleFabric::new("test-rule", "Test rule", Category::Heading);
        let registry = rule_fabric_registry![fabric];

        let seed = RulesSeed::new(&registry);
        let json = r#"{"test-rule":{"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]}}}"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("missing field `enable`"));
    }

    #[test]
    #[traced_test]
    fn deserialize_missing_config() {
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let seed = RulesSeed::new(&registry);
        let json = r#"{"test-rule":{"enable":true}}"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("missing field `config`"));
    }

    #[test]
    #[traced_test]
    fn deserialize_duplicate_fields() {
        let fabric = TestRuleFabric::default();
        let registry = rule_fabric_registry![fabric];

        let seed = RulesSeed::new(&registry);
        let json = r#"{"test-rule":{"enable":true,"enable":false,"config":{}}}"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("duplicate field `enable`"));
    }

    #[test]
    #[traced_test]
    fn deserialize_unknown_wrapper_field() {
        let fabric = TestRuleFabric::new("test-rule", "Test rule", Category::Heading);
        let registry = rule_fabric_registry![fabric];

        let seed = RulesSeed::new(&registry);
        let json = r#"{"test-rule":{"enable":true,"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]},"timeout":50"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("unknown field `timeout`"));
    }

    #[test]
    #[traced_test]
    fn deserialize_duplicate_rule() {
        let fabric = TestRuleFabric::new("test-rule", "Test rule", Category::Heading);
        let registry = rule_fabric_registry![fabric];

        let seed = RulesSeed::new(&registry);
        let json = r#"{"test-rule":{"enable":false,"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]}},
            "test-rule":{"enable":false,"config":{"name":"test-rule","description":"Test rule","category":"heading","check_result":[]}}}"#;

        let error = seed
            .deserialize(&mut serde_json::Deserializer::from_str(json))
            .unwrap_err();

        assert!(error.to_string().contains("Duplicate rule"));
    }
}
