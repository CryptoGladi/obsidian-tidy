use super::Lints;
use crate::lint::{Category, DynLint, ToggleableLint};
use ::serde::{Deserialize, Serialize, Serializer};
use serde::de::DeserializeSeed;
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use tracing::{instrument, trace};

#[derive(Debug, Serialize, Deserialize)]
pub struct LintConfig {
    pub enable: bool,
}

type LintName = String;
type CategoryLints = BTreeMap<LintName, LintConfig>;

/// Impl Lints for serde
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SerdeLints(BTreeMap<Category, CategoryLints>);

impl SerdeLints {
    #[instrument]
    fn add_lint(&mut self, category: Category, name: LintName, enable: bool) {
        trace!("Add lint");

        self.entry(category)
            .or_default()
            .insert(name, LintConfig { enable });
    }
}

impl Deref for SerdeLints {
    type Target = BTreeMap<Category, CategoryLints>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerdeLints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for Lints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut lints = SerdeLints::default();

        for lint in &self.0 {
            lints.add_lint(lint.category(), lint.name().to_string(), lint.enabled());
        }

        serializer.serialize_newtype_struct("lints", &lints)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LintsDeserializer<'a> {
    available_lints: &'a Vec<DynLint>,
    lints_from_config: &'a SerdeLints,
}

impl<'a> LintsDeserializer<'a> {
    pub fn new(available_lints: &'a Vec<DynLint>, lints_from_config: &'a SerdeLints) -> Self {
        Self {
            available_lints,
            lints_from_config,
        }
    }

    pub fn deserialise(&self) -> Result<Lints, crate::lint::Error> {
        let mut vec_lints = Vec::with_capacity(self.available_lints.len());

        for (_category, map) in &self.lints_from_config.0 {
            for (name, config) in map {
                let lint = self
                    .available_lints
                    .iter()
                    .find(|lint| lint.name() == name)
                    .unwrap();

                vec_lints.push(ToggleableLint::new(lint.clone(), config.enable));
            }
        }

        Lints::new(vec_lints)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for LintsDeserializer<'a> {
    type Value = Lints;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!("EXA")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::lint::ToggleableLint;
    use crate::test_utils::TestLint;

    #[test]
    fn serialize() {
        let lint1 = ToggleableLint::new(Arc::new(TestLint::new("lint1", Category::Content)), true);
        let lint2 = ToggleableLint::new(Arc::new(TestLint::new("lint2", Category::Spacing)), false);

        let lints = Lints::new(vec![lint1, lint2]).unwrap();
        let toml = toml::to_string(&lints).unwrap();

        assert_eq!(
            toml,
            r#"[content.lint1]
enable = true

[spacing.lint2]
enable = false
"#
        );
    }
}
