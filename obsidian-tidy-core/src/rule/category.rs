use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Yaml,
    Heading,
    Content,
    Spacing,
    Other,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Yaml => write!(f, "Yaml"),
            Category::Heading => write!(f, "Heading"),
            Category::Content => write!(f, "Content"),
            Category::Spacing => write!(f, "Spacing"),
            Category::Other => write!(f, "Other"),
        }
    }
}
