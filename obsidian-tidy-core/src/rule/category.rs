use derive_more::IsVariant;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    IsVariant,
    EnumIter,
    strum::Display,
)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Yaml,
    Heading,
    Content,
    Spacing,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn is_variant() {
        assert!(Category::Heading.is_heading());
        assert!(!Category::Content.is_heading());

        assert_eq!(
            Category::iter()
                .filter(|category| category.is_heading())
                .count(),
            1
        );
    }

    #[test]
    fn display() {
        assert_eq!(Category::Heading.to_string(), "Heading");
    }
}
