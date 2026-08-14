use super::{Node, NodeKind, Tag};
use std::borrow::Cow;

/// Trait for extracting plain text content from AST nodes.
pub trait TextContent<'a> {
    /// Returns plain text if the node contains exactly one text child with no formatting.
    ///
    /// Returns `None` if:
    /// - There's formatting (bold, italic, etc.)
    /// - There are multiple children
    /// - The node is empty
    fn as_plain_text(&self) -> Option<&Cow<'a, str>>;
}

impl<'a, T> TextContent<'a> for Tag<'a, T> {
    fn as_plain_text(&self) -> Option<&Cow<'a, str>> {
        match self.children() {
            [node] => node.as_text(),
            _ => None,
        }
    }
}

impl<'a> TextContent<'a> for Node<'a> {
    fn as_plain_text(&self) -> Option<&Cow<'a, str>> {
        match self.kind() {
            NodeKind::Text(text) => Some(text),
            _ => None,
        }
    }
}
