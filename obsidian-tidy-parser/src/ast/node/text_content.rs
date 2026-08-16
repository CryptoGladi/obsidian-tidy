use super::{Node, NodeKind, Tag};

/// Trait for extracting plain text content from AST nodes.
pub trait TextContent<'ast> {
    /// Returns plain text if the node contains exactly one text child,
    /// possibly wrapped in intermediate containers (Paragraph, Strong, etc.).
    ///
    /// Returns `None` if:
    /// - There's formatting that changes meaning (multiple children)
    /// - The node is empty
    fn as_plain_text(&'ast self) -> Option<&'ast str>;
}

impl<'ast, T> TextContent<'ast> for Tag<'ast, T> {
    fn as_plain_text(&'ast self) -> Option<&'ast str> {
        match self.children() {
            [node] => node.as_plain_text(),
            _ => None,
        }
    }
}

impl<'ast> TextContent<'ast> for Node<'ast> {
    fn as_plain_text(&'ast self) -> Option<&'ast str> {
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
