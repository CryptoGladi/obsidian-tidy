use super::{Node, NodeKind, Tag};

/// Trait for extracting plain text content from AST nodes.
pub trait TextContent {
    /// Returns plain text if the node contains exactly one text child,
    /// possibly wrapped in intermediate containers (Paragraph, Strong, etc.).
    ///
    /// Returns `None` if:
    /// - There's formatting that changes meaning (multiple children)
    /// - The node is empty
    fn as_plain_text(&self) -> Option<&str>;
}

impl<T> TextContent for Tag<'_, T> {
    fn as_plain_text(&self) -> Option<&str> {
        match self.children() {
            [node] => node.as_plain_text(),
            _ => None,
        }
    }
}

impl TextContent for Node<'_> {
    fn as_plain_text(&self) -> Option<&str> {
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
