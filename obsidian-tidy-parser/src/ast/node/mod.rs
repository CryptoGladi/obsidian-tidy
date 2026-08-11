mod heading;
mod paragraph;

pub use heading::Heading;
pub use paragraph::Paragraph;

use crate::prelude::CowStr;
use derive_more::IsVariant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag<'a, T> {
    kind: T,
    children: Box<[Node<'a>]>,
}

impl<'a, T> Tag<'a, T> {
    pub fn new(kind: T, children: Box<[Node<'a>]>) -> Self {
        Self { kind, children }
    }

    pub fn children(&self) -> &[Node<'a>] {
        &self.children
    }
}

#[derive(Debug, Clone, PartialEq, Eq, IsVariant)]
pub enum Node<'a> {
    Root(Box<[Node<'a>]>),

    Paragraph(Tag<'a, Paragraph>),
    Heading(Tag<'a, Heading>),

    Text(CowStr<'a>),
}

impl<'a> Node<'a> {
    pub fn as_root(&self) -> Option<&[Node<'a>]> {
        if let Node::Root(data) = self {
            return Some(data);
        }

        None
    }
}
