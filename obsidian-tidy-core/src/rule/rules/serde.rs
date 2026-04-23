use super::Rules;
use crate::rule::RuleFabricRegistry;
use serde::{
    Deserializer, Serialize, Serializer,
    de::{DeserializeSeed, Error, Visitor},
    ser::SerializeMap,
};

impl Serialize for Rules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        #[derive(Serialize)]
        struct InnerValue<'a, T: Serialize> {
            enable: bool,

            config: &'a T,
        }

        for (name, lint) in self.iter() {
            map.serialize_entry(
                name,
                &InnerValue {
                    enable: lint.is_enabled(),
                    config: lint.as_rule(),
                },
            )?;
        }

        map.end()
    }
}

struct RulesSeed {
    registry: RuleFabricRegistry,
}

impl<'de> DeserializeSeed<'de> for RulesSeed {
    type Value = Rules;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RulesVisitor<'a> {
            registry: &'a RuleFabricRegistry,
        }

        impl<'de, 'a> Visitor<'de> for RulesVisitor<'a> {
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
                let mut rule = Rules::new();

                while let Some(rule_name) = map.next_key::<String>()? {
                    let fabric = self
                        .registry
                        .get(&rule_name)
                        .ok_or(M::Error::custom(format!(
                            "Unknown rule '{}'. Available: {:?}",
                            rule_name,
                            self.registry.names().collect::<Vec<_>>()
                        )))?;

                    //map.next_value_seed(seed);
                    // TODO
                }

                todo!()
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {}
