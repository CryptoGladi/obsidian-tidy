use super::{Node, NodeKind, Tag};
use pulldown_cmark::HeadingLevel as MarkHeadingLevel;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<MarkHeadingLevel> for HeadingLevel {
    fn from(level: MarkHeadingLevel) -> Self {
        match level {
            MarkHeadingLevel::H1 => HeadingLevel::H1,
            MarkHeadingLevel::H2 => HeadingLevel::H2,
            MarkHeadingLevel::H3 => HeadingLevel::H3,
            MarkHeadingLevel::H4 => HeadingLevel::H4,
            MarkHeadingLevel::H5 => HeadingLevel::H5,
            MarkHeadingLevel::H6 => HeadingLevel::H6,
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

super::impl_node_as!(Heading);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::node::TextContent;
    use crate::prelude::Parser;

    #[test]
    fn display_all_variants() {
        let cases = [
            (HeadingLevel::H1, "H1"),
            (HeadingLevel::H2, "H2"),
            (HeadingLevel::H3, "H3"),
            (HeadingLevel::H4, "H4"),
            (HeadingLevel::H5, "H5"),
            (HeadingLevel::H6, "H6"),
        ];

        for (level, expected) in cases {
            assert_eq!(
                level.to_string(),
                expected,
                "Failed formatting for {:?}",
                level
            );
        }
    }

    #[test]
    fn from_pulldown_cmark_heading_level() {
        let cases = [
            (MarkHeadingLevel::H1, HeadingLevel::H1),
            (MarkHeadingLevel::H2, HeadingLevel::H2),
            (MarkHeadingLevel::H3, HeadingLevel::H3),
            (MarkHeadingLevel::H4, HeadingLevel::H4),
            (MarkHeadingLevel::H5, HeadingLevel::H5),
            (MarkHeadingLevel::H6, HeadingLevel::H6),
        ];

        for (level, expected) in cases {
            assert_eq!(HeadingLevel::from(level), expected);
        }
    }

    #[test]
    fn parse() {
        let document = "# Definition\nRust is one of the memory-safe programming languages";
        let ast = Parser::new(document).ast();

        assert_eq!(ast.count(|node| node.kind().is_heading()), 1);
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    fn parse_with_format() {
        let document = "# **Super** `Definition`\nSimple text";
        let ast = Parser::new(document).ast();

        assert_eq!(ast.count(|node| node.kind().is_heading()), 1);
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    fn as_plain_text() {
        let document = "# Simple heading";
        let ast = Parser::new(document).ast();

        // TODO операция find лучше всего
        let heading = ast.as_root().unwrap().children().first().unwrap();
        assert_eq!(
            heading.as_heading().unwrap().as_plain_text().unwrap(),
            "Simple heading"
        );
    }
}
