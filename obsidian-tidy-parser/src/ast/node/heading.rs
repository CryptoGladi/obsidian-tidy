use super::{Node, NodeKind, Tag};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<pulldown_cmark::HeadingLevel> for HeadingLevel {
    fn from(level: pulldown_cmark::HeadingLevel) -> Self {
        match level {
            pulldown_cmark::HeadingLevel::H1 => HeadingLevel::H1,
            pulldown_cmark::HeadingLevel::H2 => HeadingLevel::H2,
            pulldown_cmark::HeadingLevel::H3 => HeadingLevel::H3,
            pulldown_cmark::HeadingLevel::H4 => HeadingLevel::H4,
            pulldown_cmark::HeadingLevel::H5 => HeadingLevel::H5,
            pulldown_cmark::HeadingLevel::H6 => HeadingLevel::H6,
        }
    }
}

impl std::fmt::Display for HeadingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "H{}", *self as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Heading {
    level: HeadingLevel,
}

impl Heading {
    #[must_use]
    pub const fn new(level: HeadingLevel) -> Self {
        Self { level }
    }

    #[must_use]
    pub const fn level(&self) -> HeadingLevel {
        self.level
    }
}

impl Tag<'_, Heading> {
    #[must_use]
    pub const fn level(&self) -> HeadingLevel {
        self.kind.level()
    }
}

impl Node<'_> {
    #[must_use]
    pub const fn as_heading(&self) -> Option<&Tag<'_, Heading>> {
        if let NodeKind::Heading(data) = &self.kind {
            return Some(data);
        }

        None
    }
}
