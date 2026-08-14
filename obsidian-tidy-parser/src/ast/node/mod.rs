mod heading;
mod iter;
pub(crate) mod macros;
mod paragraph;
mod range_serde;
mod root;
mod strong;
mod text_content;

pub use heading::{Heading, HeadingLevel};
pub(crate) use macros::impl_node_as;
pub use paragraph::Paragraph;
pub use root::Root;
pub use strong::Strong;
pub use text_content::TextContent;

use derive_more::IsVariant;
use serde::Serialize;
use std::borrow::Cow;
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
    Strong(Tag<'a, Strong>),

    Text(Cow<'a, str>),
    InlineCode(Cow<'a, str>),
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

    pub fn as_text(&self) -> Option<&Cow<'a, str>> {
        match &self.kind {
            NodeKind::Text(text) => Some(&text),
            _ => None,
        }
    }
}
