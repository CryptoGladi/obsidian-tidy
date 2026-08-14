use crate::Paragraph;
use crate::prelude::Tag;
use regex::Regex;
use serde::Serialize;
use std::borrow::Cow;
use std::sync::LazyLock;

static CALLOUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\[!([^\]]+)\]([+-])?(.*)$").expect("callout regex should compile")
});

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum CalloutType<'ast> {
    Tip,
    // И остальные
    Other(Cow<'ast, str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum CalloutFoldable {
    /// `[!tip]+` — развёрнутый
    Expanded,
    /// `[!tip]-` — свёрнутый
    Collapsed,
    /// `[!tip]` — обычный
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Callout<'ast> {
    pub callout_type: CalloutType<'ast>,
    pub foldable: CalloutFoldable,
    pub title: Option<Cow<'ast, str>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BlockQuote;

impl BlockQuote {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

fn combine_text(paragraph: &Tag<'_, Paragraph>) -> Option<String> {
    let children = paragraph.children();

    // TEXT + TEXT + TEXT
    let mut idx = 0usize;
    let mut parts = String::new();
    for child in children {
        if idx >= 3 {
            return Some(parts);
        }

        if let Some(text) = child.as_text() {
            parts.push_str(text);
        } else {
            return None;
        }

        idx += 1;
    }

    None
}

impl<'ast> Tag<'ast, BlockQuote> {
    pub fn as_callout(&'ast self) -> Option<Callout<'ast>> {
        let first_child = self.children().first()?;
        let paragraph = first_child.as_paragraph()?;

        let text = combine_text(paragraph)?;
        tracing::warn!(%text, "callout text");
        tracing::warn!(?paragraph, "paragraph");

        let first_line = text.lines().next()?.to_string();
        let captures = CALLOUT_REGEX.captures(&first_line)?;

        let foldable = match captures.get(2).map(|m| m.as_str()) {
            Some("+") => CalloutFoldable::Expanded,
            Some("-") => CalloutFoldable::Collapsed,
            _ => CalloutFoldable::None,
        };

        let type_str = captures.get(1)?.as_str();
        let callout_type = match type_str.to_ascii_lowercase().as_str() {
            "tip" => CalloutType::Tip,
            _ => CalloutType::Other(Cow::Owned(type_str.to_string())),
        };

        let title = captures
            .get(3)
            .map(|m| m.as_str())
            .filter(|s| !s.is_empty())
            .map(Cow::Borrowed);

        Some(Callout {
            callout_type,
            foldable,
            title,
        })
    }
}

super::impl_node_as!(BlockQuote);

#[cfg(test)]
mod tests {
    use crate::prelude::{Parser, TextContent};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn ast() {
        let document = "# Define\n>My **super** quote";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[traced_test]
    fn nested_ast() {
        let document = "# Define\n>My **super** quote\n>> Quote by quote";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[traced_test]
    fn as_plain_text() {
        let document = "# Define\n> My quote";
        let ast = Parser::new(document).ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert_eq!(quote.as_plain_text().unwrap(), "My quote");
    }

    #[test]
    #[traced_test]
    fn as_plain_text_with_formatting() {
        let document = "# Define\n> My **super** quote";
        let ast = Parser::new(document).ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert!(quote.as_plain_text().is_none());
    }

    #[test]
    #[traced_test]
    fn kind() {
        let document = "# Define\n> ![tip] My **super** quote";
        let ast = Parser::new(document).ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        let callout = quote.as_callout().unwrap();

        panic!("{:?}", callout);
    }
}
