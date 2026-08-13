use super::{Node, NodeKind, Tag};
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Paragraph;

impl Paragraph {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Node<'_> {
    #[must_use]
    pub const fn as_paragraph(&self) -> Option<&Tag<'_, Paragraph>> {
        if let NodeKind::Paragraph(data) = &self.kind {
            return Some(data);
        }

        None
    }
}
