mod callout;
mod code_block;
mod footnote_definition;
mod heading;
mod impl_enum_tag;
mod links;
mod list;
mod table;

pub use callout::{Callout, CalloutFoldable};
pub use code_block::CodeBlock;
pub use footnote_definition::*;
pub use heading::{Heading, HeadingLevel};
pub use links::*;
pub use list::{List, ListDelimiter};
pub use table::{Alignment, Table};

use impl_enum_tag::impl_enum_tag;

impl_enum_tag! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(bound(deserialize = "'input: 'de")))]
    pub enum Tag<'input> {
        Paragraph,
        Heading(Heading),
        CodeBlock(CodeBlock<'input>),
        BlockQuote,
        Callout(Callout<'input>),
        Strong,
        Strikethrough,
        Emphasis,
        HtmlBlock,
        FootnoteDefinition(FootnoteDefinition<'input>),

        List(List),
        Item,

        DefinitionList,
        DefinitionListTitle,
        DefinitionListDefinition,

        Table(Table),
        TableHead,
        TableRow,
        TableCell,

        /// It is not supported in Obsidian
        Superscript,

        /// It is not supported in Obsidian
        Subscript,

        Link(Link<'input>)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum TagEnd { ... }
}

impl<'input> Tag<'input> {
    pub fn is_inline_link(&self) -> bool {
        matches!(self, Self::Link(Link::Inline(_)))
    }

    pub fn as_inline_link(&self) -> Option<&InlineLink<'input>> {
        if let Self::Link(Link::Inline(link)) = self {
            return Some(link);
        }

        None
    }

    pub fn into_inline_link(self) -> Option<InlineLink<'input>> {
        if let Self::Link(Link::Inline(link)) = self {
            return Some(link);
        }

        None
    }

    pub fn is_autolink(&self) -> bool {
        matches!(self, Self::Link(Link::Autolink(_)))
    }

    pub fn as_autolink(&self) -> Option<&Autolink<'input>> {
        if let Self::Link(Link::Autolink(link)) = self {
            return Some(link);
        }

        None
    }

    pub fn into_autolink(self) -> Option<Autolink<'input>> {
        if let Self::Link(Link::Autolink(link)) = self {
            return Some(link);
        }

        None
    }
}

crate::__private::impl_as_target_self!(Tag<'_>, TagEnd);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    macro_rules! assert_json_snapshot {
        ($tokens:ident) => {{
            let tokens: Vec<_> = $tokens
                .into_iter()
                .map(|(token, range)| (token, std::ops::Range::from(range)))
                .collect();

            insta::assert_json_snapshot!(tokens);
        }};
    }

    fn check_start<F>(tokens: &[(Token, Range<usize>)], predicate: F) -> bool
    where
        F: FnMut(&Tag) -> bool,
    {
        let mut predicate = predicate;

        tokens
            .iter()
            .any(|(token, _)| token.as_start().map_or(false, &mut predicate))
    }

    fn check_end<F>(tokens: &[(Token, Range<usize>)], predicate: F) -> bool
    where
        F: FnMut(&TagEnd) -> bool,
    {
        let mut predicate = predicate;

        tokens
            .iter()
            .any(|(token, _)| token.as_end().map_or(false, &mut predicate))
    }

    // ⚠️ CRITICAL: Definition list require `definition_list(true)`
    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn definition_list() {
        let source = r#"title 1
  : definition 1
title 2
  : definition 2"#;

        let tokens: Vec<_> = TokenStreamBuilder::<InterceptorEnum>::new()
            .definition_list(true)
            .build(source)
            .with_tracing()
            .collect();

        assert!(!tokens.is_empty());

        assert!(check_start(&tokens, |start| start.is_definition_list()));
        assert!(check_start(&tokens, |start| start.is_definition_list_title()));
        assert!(check_start(&tokens, |start| start.is_definition_list_definition()));

        assert!(check_end(&tokens, |end| end.is_definition_list()));
        assert!(check_end(&tokens, |end| end.is_definition_list_title()));
        assert!(check_end(&tokens, |end| end.is_definition_list_definition()));

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // ⚠️ CRITICAL: Strikethrough require `strikethrough(true)`
    #[test]
    fn strikethrough() {
        // TODO
        // Error in documentation pulldown-cmark
        // <https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Tag.html#variant.Strikethrough>
        let source = "Super ~~very~~ test";
        let tokens: Vec<_> = TokenStreamBuilder::<InterceptorEnum>::new()
            .strikethrough(true)
            .build(source)
            .with_tracing()
            .collect();

        assert!(!tokens.is_empty());
        assert!(check_start(&tokens, |start| start.is_strikethrough()));
        assert!(check_end(&tokens, |end| end.is_strikethrough()));

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // ⚠️ CRITICAL: Require `superscript(true)`
    #[test]
    fn superscript() {
        let source = "f(x) = x^2^";
        let tokens: Vec<_> = TokenStreamBuilder::<InterceptorEnum>::new()
            .superscript(true)
            .build(source)
            .with_tracing()
            .collect();

        assert!(!tokens.is_empty());
        assert!(check_start(&tokens, |start| start.is_superscript()));
        assert!(check_end(&tokens, |end| end.is_superscript()));

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // ⚠️ CRITICAL: Require `subscript(true)`
    #[test]
    fn subscript() {
        let source = "~subscript~ ~~if also enabled this is strikethrough~~";
        let tokens: Vec<_> = TokenStreamBuilder::<InterceptorEnum>::new()
            .subscript(true)
            .build(source)
            .with_tracing()
            .collect();

        assert!(!tokens.is_empty());
        assert!(check_start(&tokens, |start| start.is_subscript()));
        assert!(check_end(&tokens, |end| end.is_subscript()));

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }
}
