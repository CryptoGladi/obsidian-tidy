use super::{Node, NodeKind, Tag};
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Root;

impl Root {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Node<'_> {
    #[must_use]
    pub const fn as_root(&self) -> Option<&Tag<'_, Root>> {
        if let NodeKind::Root(data) = &self.kind {
            return Some(data);
        }

        None
    }
}
