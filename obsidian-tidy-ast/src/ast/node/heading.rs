use super::Tag;
use obsidian_tidy_lexer::{Heading, HeadingLevel};

// TODO

impl Tag<'_, Heading> {
    #[must_use]
    pub const fn level(&self) -> HeadingLevel {
        self.kind.level()
    }
}

super::impl_node_as!(Heading);

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, TextContent};

    #[test]
    #[cfg_attr(miri, ignore)]
    fn parse() {
        let text = "# Definition\nRust is one of the memory-safe programming languages";
        let document = Document::new(text);
        let ast = document.ast();

        assert_eq!(ast.count(|node| node.kind().is_heading()), 1);
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn parse_with_format() {
        let text = "# **Super** `Definition`\nSimple text";
        let document = Document::new(text);
        let ast = document.ast();

        assert_eq!(ast.count(|node| node.kind().is_heading()), 1);
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    fn as_plain_text() {
        let document = Document::new("# Simple heading");
        let ast = document.ast();

        let heading: Vec<_> = ast.collect_map(|node| node.as_heading());

        assert_eq!(heading.len(), 1);
        assert_eq!(heading[0].as_plain_text().unwrap(), "Simple heading");
    }
}
