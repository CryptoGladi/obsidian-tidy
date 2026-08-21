mod block_quote;
pub mod callout;
mod heading;
mod iter;
pub(crate) mod macros;
mod paragraph;
mod root;
mod strong;
mod text_content;

pub use block_quote::BlockQuote;
pub use callout::Callout;
pub(crate) use macros::impl_node_as;
pub use paragraph::Paragraph;
pub use root::Root;
pub use strong::Strong;
pub use text_content::TextContent;

use crate::token_stream::token::Heading;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::range::Range;
use derive_more::IsVariant;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Serialize, Deserialize, Display)]
#[non_exhaustive]
pub enum NodeKind<'ast> {
    Root(Tag<'ast, Root>),

    Paragraph(Tag<'ast, Paragraph>),
    Heading(Tag<'ast, Heading>),
    Strong(Tag<'ast, Strong>),
    BlockQuote(Tag<'ast, BlockQuote>),
    Callout(Tag<'ast, Callout<'ast>>),

    Text(Cow<'ast, str>),
    InlineCode(Cow<'ast, str>),

    SoftBreak,
    HardBreak,

    Rule,

    InlineMath(Cow<'ast, str>),
    DisplayMath(Cow<'ast, str>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node<'a> {
    #[serde(flatten)]
    kind: NodeKind<'a>,

    #[serde(with = "crate::__private::range_serde")]
    offset: Range<usize>,
}

impl core::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<'ast> Node<'ast> {
    #[must_use]
    pub const fn new(kind: NodeKind<'ast>, offset: Range<usize>) -> Self {
        Self { kind, offset }
    }

    #[must_use]
    pub const fn kind(&self) -> &NodeKind<'ast> {
        &self.kind
    }

    #[must_use]
    pub const fn offset(&self) -> Range<usize> {
        self.offset
    }

    #[must_use]
    pub fn as_text(&'ast self) -> Option<&'ast str> {
        match &self.kind {
            NodeKind::Text(text) => Some(text.as_ref()),
            _ => None,
        }
    }
}
