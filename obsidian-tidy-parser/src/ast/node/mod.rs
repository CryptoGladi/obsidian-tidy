mod block_quote;
pub mod callout;
mod heading;
mod iter;
pub(crate) mod macros;
mod paragraph;
mod range_serde;
mod root;
mod strong;
mod text_content;

pub use block_quote::BlockQuote;
pub use callout::Callout;
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
    #[serde(flatten)]
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
pub enum NodeKind<'ast> {
    Root(Tag<'ast, Root>),

    Paragraph(Tag<'ast, Paragraph>),
    Heading(Tag<'ast, Heading>),
    Strong(Tag<'ast, Strong>),
    BlockQuote(Tag<'ast, BlockQuote>),
    Callout(Tag<'ast, Callout>),

    Text(Cow<'ast, str>),
    InlineCode(Cow<'ast, str>),
    SoftBreak,
    HardBreak,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Node<'a> {
    #[serde(flatten)]
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

    #[must_use]
    pub const fn as_text(&self) -> Option<&Cow<'a, str>> {
        match &self.kind {
            NodeKind::Text(text) => Some(text),
            _ => None,
        }
    }
}
