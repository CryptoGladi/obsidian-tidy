use super::Lints as VecLints;
use crate::lint::Category;
use serde::{Deserialize, Serialize, Serializer};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Serialize, Deserialize)]
struct LintConfig {
    enable: bool,
}

type LintName = String;
type CategoryLints = BTreeMap<LintName, LintConfig>;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Lints(BTreeMap<Category, CategoryLints>);

impl Lints {
    fn add_lint(&mut self, category: Category, name: LintName, enable: bool) {
        self.entry(category)
            .or_default()
            .insert(name, LintConfig { enable });
    }
}

impl Deref for Lints {
    type Target = BTreeMap<Category, CategoryLints>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for VecLints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut lints = Lints::default();

        for lint in &self.0 {
            lints.add_lint(lint.category(), lint.name().to_string(), lint.enabled());
        }

        serializer.serialize_newtype_struct("lints", &lints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::{Content, Lint, ToggleableLint, Violation};

    struct TestLint {
        name: String,
        category: Category,
    }

    impl TestLint {
        pub fn new(name: impl Into<String>, category: Category) -> Self {
            Self {
                name: name.into(),
                category,
            }
        }
    }

    impl Lint for TestLint {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            unimplemented!()
        }

        fn category(&self) -> Category {
            self.category.clone()
        }

        fn check(&self, _content: &Content) -> Vec<Violation> {
            unimplemented!()
        }
    }

    #[test]
    fn serialize() {
        let lint1 = ToggleableLint::new(Box::new(TestLint::new("lint1", Category::Content)));
        let lint2 = ToggleableLint::with_enabled(
            Box::new(TestLint::new("lint2", Category::Spacing)),
            false,
        );

        let lints = VecLints::new(vec![lint1, lint2]).unwrap();
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
