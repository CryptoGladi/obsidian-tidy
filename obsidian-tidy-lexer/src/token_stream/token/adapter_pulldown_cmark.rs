use super::{Tag, TagEnd, Token};
use crate::{
    Autolink, CodeBlock, Heading, HeadingLevel, InlineLink, Link, List, ReferenceLink, Table,
    TaskListMarker, WikiLink,
};
use alloc::borrow::Cow;
use pulldown_cmark::{
    Event as MarkEvent, LinkType as MarkLinkType, Tag as MarkTag, TagEnd as MarkTagEnd,
};

fn adapter_link<'input>(
    link: MarkLinkType,
    dest_url: Cow<'input, str>,
    title: Cow<'input, str>,
    id: Cow<'input, str>,
) -> Link<'input> {
    #[cfg(feature = "tracing")]
    tracing::warn!(?link, ?dest_url, ?title, ?id, "adapter_link");

    let clean_title = (!title.is_empty()).then(|| title);
    let clean_destination = (!dest_url.is_empty()).then(|| dest_url.clone());

    match link {
        MarkLinkType::Inline => Link::Inline(InlineLink {
            destination: dest_url,
            title: clean_title,
        }),
        MarkLinkType::Reference => Link::Reference(ReferenceLink {
            reference: id,
            is_known: true,
            destination: clean_destination,
            title: clean_title,
        }),
        MarkLinkType::ReferenceUnknown => Link::Reference(ReferenceLink {
            reference: id,
            is_known: false,
            destination: None,
            title: None,
        }),
        MarkLinkType::Collapsed => Link::Reference(ReferenceLink {
            reference: id,
            is_known: true,
            destination: clean_destination,
            title: clean_title,
        }),
        MarkLinkType::CollapsedUnknown => Link::Reference(ReferenceLink {
            reference: id,
            is_known: true,
            destination: clean_destination,
            title: clean_title,
        }),
        MarkLinkType::Shortcut => Link::Reference(ReferenceLink {
            reference: id,
            is_known: true,
            destination: clean_destination,
            title: clean_title,
        }),
        MarkLinkType::ShortcutUnknown => Link::Reference(ReferenceLink {
            reference: id,
            is_known: false,
            destination: None,
            title: None,
        }),
        MarkLinkType::Autolink => Link::Autolink(Autolink {
            url: dest_url,
            is_email: false,
        }),
        MarkLinkType::Email => Link::Autolink(Autolink {
            url: dest_url,
            is_email: true,
        }),
        MarkLinkType::WikiLink { has_pothole } => Link::WikiLink(WikiLink {
            destination: dest_url,
            has_pothole,
        }),
    }
}

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
            MarkTag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Link(adapter_link(
                link_type,
                dest_url.into(),
                title.into(),
                id.into(),
            )),
            MarkTag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => Tag::Link(adapter_link(
                link_type,
                dest_url.into(),
                title.into(),
                id.into(),
            )),
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
            MarkTagEnd::Link | MarkTagEnd::Image => TagEnd::Link,
            _ => todo!("{tag_end:?}"),
        }
    }
}
