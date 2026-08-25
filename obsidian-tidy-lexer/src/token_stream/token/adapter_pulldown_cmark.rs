use super::{Tag, TagEnd, Token};
use crate::{CodeBlock, Heading, HeadingLevel, List, Table, TaskListMarker};
use pulldown_cmark::{Event as MarkEvent, Tag as MarkTag, TagEnd as MarkTagEnd};

impl<'input> From<pulldown_cmark::Event<'input>> for Token<'input> {
    fn from(event: pulldown_cmark::Event<'input>) -> Self {
        match event {
            MarkEvent::Start(tag_start) => Token::Start(tag_start.into()),
            MarkEvent::End(tag_end) => Token::End(tag_end.into()),
            MarkEvent::Text(text) => Token::Text(text.into()),
            MarkEvent::SoftBreak => Token::SoftBreak,
            MarkEvent::HardBreak => Token::HardBreak,
            MarkEvent::Code(lang) => Token::Code(lang.into()),
            MarkEvent::Rule => Token::Rule,
            MarkEvent::InlineMath(text) => Token::InlineMath(text.into()),
            MarkEvent::DisplayMath(text) => Token::DisplayMath(text.into()),
            MarkEvent::Html(text) => Token::Html(text.into()),
            MarkEvent::InlineHtml(text) => Token::InlineHtml(text.into()),
            MarkEvent::FootnoteReference(text) => Token::FootnoteReference(text.into()),
            MarkEvent::TaskListMarker(is_done) => {
                Token::TaskListMarker(TaskListMarker::new(is_done))
            }
        }
    }
}

impl<'input> From<pulldown_cmark::Tag<'input>> for Tag<'input> {
    fn from(tag: pulldown_cmark::Tag<'input>) -> Self {
        match tag {
            MarkTag::Paragraph => Tag::Paragraph,
            MarkTag::Heading { level, .. } => Tag::Heading(Heading::new(HeadingLevel::from(level))),
            MarkTag::CodeBlock(block) => Tag::CodeBlock(CodeBlock::from(block)),
            MarkTag::BlockQuote(quote) => {
                // We have disabled this option!
                debug_assert!(quote.is_none());

                Tag::BlockQuote
            }
            MarkTag::Strong => Tag::Strong,
            MarkTag::Emphasis => Tag::Emphasis,
            MarkTag::Strikethrough => Tag::Strikethrough,
            MarkTag::List(number_item) => Tag::List(List::new(number_item)),
            MarkTag::Item => Tag::Item,
            MarkTag::HtmlBlock => Tag::HtmlBlock,
            MarkTag::DefinitionList => Tag::DefinitionList,
            MarkTag::DefinitionListTitle => Tag::DefinitionListTitle,
            MarkTag::DefinitionListDefinition => Tag::DefinitionListDefinition,
            MarkTag::Table(alignmant) => Tag::Table(Table::from(alignmant)),
            MarkTag::TableHead => Tag::TableHead,
            MarkTag::TableRow => Tag::TableRow,
            MarkTag::TableCell => Tag::TableCell,
            MarkTag::Superscript => Tag::Superscript,
            MarkTag::Subscript => Tag::Subscript,
            _ => todo!("{tag:?}"),
        }
    }
}

impl From<pulldown_cmark::TagEnd> for TagEnd {
    fn from(tag_end: pulldown_cmark::TagEnd) -> Self {
        match tag_end {
            MarkTagEnd::Paragraph => TagEnd::Paragraph,
            MarkTagEnd::Heading(_) => TagEnd::Heading,
            MarkTagEnd::CodeBlock => TagEnd::CodeBlock,
            MarkTagEnd::BlockQuote(quote) => {
                // Мы отключили эту опцию!
                debug_assert!(quote.is_none());

                TagEnd::BlockQuote
            }
            MarkTagEnd::Strong => TagEnd::Strong,
            MarkTagEnd::Emphasis => TagEnd::Emphasis,
            MarkTagEnd::Strikethrough => TagEnd::Strikethrough,
            MarkTagEnd::List(_) => TagEnd::List,
            MarkTagEnd::Item => TagEnd::Item,
            MarkTagEnd::HtmlBlock => TagEnd::HtmlBlock,
            MarkTagEnd::DefinitionList => TagEnd::DefinitionList,
            MarkTagEnd::DefinitionListTitle => TagEnd::DefinitionListTitle,
            MarkTagEnd::DefinitionListDefinition => TagEnd::DefinitionListDefinition,
            MarkTagEnd::Table => TagEnd::Table,
            MarkTagEnd::TableHead => TagEnd::TableHead,
            MarkTagEnd::TableRow => TagEnd::TableRow,
            MarkTagEnd::TableCell => TagEnd::TableCell,
            MarkTagEnd::Superscript => TagEnd::Superscript,
            MarkTagEnd::Subscript => TagEnd::Subscript,
            _ => todo!("{tag_end:?}"),
        }
    }
}
