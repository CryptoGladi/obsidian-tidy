mod heading;
mod paragraph;
mod root;

pub use heading::Heading;
pub use paragraph::Paragraph;
pub use root::Root;

use crate::prelude::CowStr;
use derive_more::IsVariant;
use std::range::Range;

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
pub enum NodeKind<'a> {
    Root(Tag<'a, Root>),

    Paragraph(Tag<'a, Paragraph>),
    Heading(Tag<'a, Heading>),

    Text(CowStr<'a>),
    SoftBreak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'a> {
    pub kind: NodeKind<'a>,
    pub offset: Range<usize>,
}

impl<'a> Node<'a> {
    pub fn new(kind: NodeKind<'a>, offset: Range<usize>) -> Self {
        Self { kind, offset }
    }

    pub fn as_root(&self) -> Option<&Tag<'a, Root>> {
        if let NodeKind::Root(data) = &self.kind {
            return Some(data);
        }

        None
    }
}
