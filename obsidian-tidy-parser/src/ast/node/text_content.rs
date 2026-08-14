use super::{Node, NodeKind, Tag};
use std::borrow::Cow;

/// Trait for extracting plain text content from AST nodes.
pub trait TextContent<'a> {
    /// Returns plain text if the node contains exactly one text child,
    /// possibly wrapped in intermediate containers (Paragraph, Strong, etc.).
    ///
    /// Returns `None` if:
    /// - There's formatting that changes meaning (multiple children)
    /// - The node is empty
    fn as_plain_text(&self) -> Option<&Cow<'a, str>>;
}

impl<'a, T> TextContent<'a> for Tag<'a, T> {
    fn as_plain_text(&self) -> Option<&Cow<'a, str>> {
        match self.children() {
            [node] => node.as_plain_text(),
            _ => None,
        }
    }
}

impl<'a> TextContent<'a> for Node<'a> {
    fn as_plain_text(&self) -> Option<&Cow<'a, str>> {
        match self.kind() {
            NodeKind::Text(text) => Some(text),
            NodeKind::Root(tag) => tag.as_plain_text(),
            NodeKind::Paragraph(tag) => tag.as_plain_text(),
            NodeKind::Heading(tag) => tag.as_plain_text(),
            NodeKind::Strong(tag) => tag.as_plain_text(),
            _ => None,
        }
    }
}
