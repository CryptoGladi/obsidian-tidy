mod callout;
mod code_block;
mod heading;
mod impl_enum_tag;
mod list;

pub use callout::{Callout, CalloutFoldable};
pub use code_block::CodeBlock;
pub use heading::{Heading, HeadingLevel};
use impl_enum_tag::impl_enum_tag;
pub use list::{List, ListDelimiter};

impl_enum_tag! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub enum Tag<'input> {
        Paragraph,
        Heading(Heading),
        CodeBlock(CodeBlock<'input>),
        BlockQuote,
        Callout(Callout<'input>),
        Strong,
        Emphasis,
        HtmlBlock,

        List(List),
        Item
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub enum TagEnd { ... }
}
