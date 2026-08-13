mod heading;
mod paragraph;
mod range_serde;
mod root;

pub use heading::Heading;
pub use paragraph::Paragraph;
pub use root::Root;

use crate::prelude::CowStr;
use derive_more::IsVariant;
use serde::Serialize;
use std::range::Range;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tag<'a, T> {
    kind: T,
    children: Box<[Node<'a>]>,
}

impl<'a, T> Tag<'a, T> {
    pub const fn new(kind: T, children: Box<[Node<'a>]>) -> Self {
        Self { kind, children }
    }

    pub fn children(&self) -> &[Node<'a>] {
        &self.children
    }
}

#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Serialize, Display)]
#[non_exhaustive]
pub enum NodeKind<'a> {
    Root(Tag<'a, Root>),

    Paragraph(Tag<'a, Paragraph>),
    Heading(Tag<'a, Heading>),

    Text(CowStr<'a>),
    SoftBreak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Node<'a> {
    kind: NodeKind<'a>,

    #[serde(with = "range_serde")]
    offset: Range<usize>,
}

impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<'a> Node<'a> {
    #[must_use]
    pub const fn new(kind: NodeKind<'a>, offset: Range<usize>) -> Self {
        Self { kind, offset }
    }

    #[must_use]
    pub const fn kind(&self) -> &NodeKind<'a> {
        &self.kind
    }

    #[must_use]
    pub const fn offset(&self) -> Range<usize> {
        self.offset
    }
}
