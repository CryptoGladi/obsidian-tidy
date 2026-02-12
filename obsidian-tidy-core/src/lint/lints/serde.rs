use super::Lints;
use crate::lint::{Category, DynLint, ToggleableLint};
use ::serde::{Deserialize, Serialize, Serializer};
use serde::{Deserializer, de::DeserializeSeed};
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
pub struct InnerLints(BTreeMap<Category, CategoryLints>);

impl InnerLints {
    #[instrument]
    fn add_lint(&mut self, category: Category, name: LintName, enable: bool) {
        trace!("Add lint");

        self.entry(category)
            .or_default()
            .insert(name, LintConfig { enable });
    }
}

impl Deref for InnerLints {
    type Target = BTreeMap<Category, CategoryLints>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InnerLints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> Serialize for Lints<E>
where
    E: std::error::Error,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut lints = InnerLints::default();

        for lint in &self.0 {
            lints.add_lint(lint.category(), lint.name().to_string(), lint.enabled());
        }

        serializer.serialize_newtype_struct("lints", &lints)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LintsSeed<'a, E>
where
    E: std::error::Error,
{
    available_lints: &'a Vec<DynLint<E>>,
}

impl<'a, E> LintsSeed<'a, E>
where
    E: std::error::Error,
{
    pub fn new(available_lints: &'a Vec<DynLint<E>>) -> Self {
        Self { available_lints }
    }
}

impl<'de, E> DeserializeSeed<'de> for LintsSeed<'_, E>
where
    E: std::error::Error,
{
    type Value = Lints<E>;

    #[instrument(skip(deserializer), err)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        trace!("Run deserialize");

        let inner = InnerLints::deserialize(deserializer)?;
        let mut vec_lints = Vec::with_capacity(self.available_lints.len());

        for (_category, map) in &inner.0 {
            for (name, config) in map {
                let lint = self
                    .available_lints
                    .iter()
                    .find(|lint| lint.name() == name)
                    .ok_or(serde::de::Error::missing_field(
                        "not found lint from `available_lints`",
                    ))?;

                vec_lints.push(ToggleableLint::new(lint.clone(), config.enable));
            }
        }

        Lints::new(vec_lints).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::ToggleableLint;
    use crate::test_utils::TestLint;
    use std::sync::Arc;

    #[test]
    fn serialize() {
        let lint1 = ToggleableLint::new(
            Arc::new(TestLint::new("lint1", "", Category::Content)),
            true,
        );
        let lint2 = ToggleableLint::new(
            Arc::new(TestLint::new("lint2", "", Category::Spacing)),
            false,
        );

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

    #[test]
    fn deserialize() {
        let lint1 = Arc::new(TestLint::new("lint1", "", Category::Content));
        let lint2 = Arc::new(TestLint::new("lint2", "", Category::Spacing));

        let toggleable_lint1 = ToggleableLint::new(lint1.clone(), true);
        let toggleable_lint2 = ToggleableLint::new(lint2.clone(), false);

        let lints = Lints::new(vec![toggleable_lint1, toggleable_lint2]).unwrap();
        let toml = toml::to_string(&lints).unwrap();

        let available_lints: Vec<DynLint<_>> = vec![lint1, lint2];
        let lints_deserialized = LintsSeed::new(&available_lints)
            .deserialize(toml::Deserializer::parse(&toml).unwrap())
            .unwrap();

        assert_eq!(lints, lints_deserialized);
    }

    #[test]
    #[should_panic]
    fn deserialize_with_not_found_lint() {
        let lint1 = Arc::new(TestLint::new("lint1", "", Category::Content));
        let lint2 = Arc::new(TestLint::new("lint2", "", Category::Spacing));

        let toggleable_lint1 = ToggleableLint::new(lint1.clone(), true);
        let toggleable_lint2 = ToggleableLint::new(lint2.clone(), false);

        let lints = Lints::new(vec![toggleable_lint1, toggleable_lint2]).unwrap();
        let toml = toml::to_string(&lints).unwrap();

        let available_lints: Vec<DynLint<_>> = vec![lint1]; // Without lint2
        let lints_deserialized = LintsSeed::new(&available_lints)
            .deserialize(toml::Deserializer::parse(&toml).unwrap())
            .unwrap();

        assert_eq!(lints, lints_deserialized);
    }
}
