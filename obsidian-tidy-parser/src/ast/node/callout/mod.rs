mod kind;

pub use kind::CalloutKind;

use super::BlockQuote;
use crate::prelude::{Node, NodeKind, Tag};
use derive_more::IsVariant;
use serde::Serialize;
use std::{borrow::Cow, iter::Peekable, range::Range};

const EXCEPT_MESSAGE: &str = "Callout::parse should have validated structure";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, IsVariant, Serialize)]
pub enum CalloutFoldable {
    /// `[!tip]+` — развёрнутый
    Expanded,

    /// `[!tip]-` — свёрнутый
    Collapsed,

    /// `[!tip]` — обычный
    #[default]
    None,
}

impl From<char> for CalloutFoldable {
    fn from(value: char) -> Self {
        match value {
            '+' => CalloutFoldable::Expanded,
            '-' => CalloutFoldable::Collapsed,
            _ => CalloutFoldable::None,
        }
    }
}

impl CalloutFoldable {
    fn from_paragraph_iter<'ast, I>(paragraph_iter: &mut Peekable<I>) -> Option<Self>
    where
        I: Iterator<Item = &'ast Node<'ast>>,
    {
        let text = paragraph_iter.peek()?;
        let char = text.as_text()?.chars().next()?;

        let foldable = CalloutFoldable::from(char);

        if !foldable.is_none() {
            paragraph_iter.next();
        }

        Some(foldable)
    }
}

pub struct CalloutContentIter<'ast> {
    iter: std::slice::Iter<'ast, Node<'ast>>,
    skip_first_char: bool,
}

impl<'ast> CalloutContentIter<'ast> {
    const fn new(iter: std::slice::Iter<'ast, Node<'ast>>, skip_first_char: bool) -> Self {
        Self {
            iter,
            skip_first_char,
        }
    }
}

impl<'ast> Iterator for CalloutContentIter<'ast> {
    type Item = Cow<'ast, Node<'ast>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.iter.next()?;

        if self.skip_first_char {
            self.skip_first_char = false;

            if let NodeKind::Text(text) = node.kind()
                && let Some(stripped) = text.strip_prefix(['+', '-'])
            {
                let new_text = Cow::Owned(stripped.to_string());
                let new_offset = (node.offset().start + 1)..node.offset().end;

                return Some(Cow::Owned(Node::new(
                    NodeKind::Text(new_text),
                    new_offset.into(),
                )));
            }
        }

        Some(Cow::Borrowed(node))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Callout<'ast> {
    pub foldable: CalloutFoldable,
    kind: CalloutKind<'ast>,
}

impl<'ast> Callout<'ast> {
    pub(crate) fn parse(
        block_quote: &Tag<'ast, BlockQuote>,
        offset: Range<usize>,
    ) -> Option<Tag<'ast, Callout<'ast>>> {
        let first_child = block_quote.children().first()?;
        let paragraph = first_child.as_paragraph()?;
        let paragraph_offset = first_child.offset();

        let whitespace_after_marker = paragraph_offset.start - offset.start;
        if whitespace_after_marker > 2 {
            return None;
        }

        let mut paragraph_iter = paragraph.children().iter().peekable();

        // Text("[") + Text("!tip") + Text("]")) + Text("+ ...")
        paragraph_iter
            .next()?
            .as_text()
            .filter(|text| *text == "[")?;

        // For check iterator
        let kind = CalloutKind::from_paragraph_iter(&mut paragraph_iter)?;

        paragraph_iter
            .next()?
            .as_text()
            .filter(|text| text.starts_with(']'))?;

        if let Some(node) = paragraph_iter.peek() {
            match node.kind() {
                NodeKind::Text(text) if text.starts_with([' ', '+', '-']) => {}
                NodeKind::SoftBreak | NodeKind::HardBreak => {}
                _ => return None,
            }
        }

        let foldable =
            CalloutFoldable::from_paragraph_iter(&mut paragraph_iter).unwrap_or_default();

        let tag = Callout { foldable };
        let callout = Tag::new(tag, block_quote.children().into());

        Some(callout)
    }
}

impl<'ast> Tag<'ast, Callout> {
    #[expect(
        clippy::expect_used,
        clippy::missing_panics_doc,
        reason = "Callout::parse should have validated structure"
    )]
    #[must_use]
    pub fn content(&'ast self) -> CalloutContentIter<'ast> {
        let first_child = self.children().first().expect(EXCEPT_MESSAGE);
        let paragraph = first_child.as_paragraph().expect(EXCEPT_MESSAGE);
        let mut paragraph_iter = paragraph.children().iter();

        // Skip Text("[")
        // Skip Text("!tip")
        // Skip Text("]")
        paragraph_iter.nth(2);

        let skip_first_char = !self.kind.foldable.is_none();
        CalloutContentIter::new(paragraph_iter, skip_first_char)
    }

    #[expect(
        clippy::expect_used,
        clippy::missing_panics_doc,
        reason = "Callout::parse should have validated structure"
    )]
    #[must_use]
    pub fn kind(&'ast self) -> CalloutKind<'ast> {
        let first_child = self.children().first().expect(EXCEPT_MESSAGE);
        let paragraph = first_child.as_paragraph().expect(EXCEPT_MESSAGE);
        let mut paragraph_iter = paragraph.children().iter().peekable();

        paragraph_iter.next(); // Ignore '['
        CalloutKind::from_paragraph_iter(&mut paragraph_iter).expect(EXCEPT_MESSAGE)
    }

    #[must_use]
    pub const fn foldable(&self) -> CalloutFoldable {
        self.kind.foldable
    }

    /// Возвращает title callout (всё после `[!type][+/-]`), trimmed.
    ///
    /// Возвращает `None` если:
    /// - Нет контента после header
    /// - Первый узел контента не Text
    /// - Title пустой после trim
    #[expect(
        clippy::expect_used,
        clippy::missing_panics_doc,
        reason = "Callout::parse should have validated structure"
    )]
    #[must_use]
    pub fn title(&'ast self) -> Option<&'ast str> {
        let first_child = self.children().first().expect(EXCEPT_MESSAGE);
        let paragraph = first_child.as_paragraph().expect(EXCEPT_MESSAGE);
        let children = paragraph.children();

        // Пропускаем Text[, !type, ]
        let content_start = 3;
        if children.len() <= content_start {
            return None;
        }

        let first_content = &children[content_start];
        let text = first_content.as_text()?;

        let text = if !self.kind.foldable.is_none() {
            text.strip_prefix(['+', '-'])?
        } else {
            text
        };

        let trimmed = text.trim();
        Some(trimmed)
    }
}

super::impl_node_as!(Callout);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Parser, TextContent};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn without_title() {
        let document = "> [!tip]\nText";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.kind(), CalloutKind::Tip);

        // Skip SoftBreak
        assert_eq!(callout.title(), None);
        assert_eq!(
            callout.content().nth(1).unwrap().as_plain_text().unwrap(),
            "Text"
        );
    }

    #[test]
    #[traced_test]
    fn many_space() {
        let document = ">  [!tip] Text";
        let ast = Parser::new(document).ast();

        assert!(ast.find_map(|node| node.as_callout()).is_none());
    }

    #[test]
    #[traced_test]
    fn zero_space() {
        let document = ">[!example]+ Text\n> Other Data";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::Expanded);
        assert_eq!(callout.kind(), CalloutKind::Example);

        assert_eq!(callout.title().unwrap(), "Text");
        assert_eq!(
            callout.content().next().unwrap().as_plain_text().unwrap(),
            " Text"
        );
    }

    #[test]
    #[traced_test]
    fn without_text() {
        let document = "> [!warning]";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.kind(), CalloutKind::Warning);

        assert_eq!(callout.title(), None);
        assert_eq!(callout.content().next(), None);
    }

    #[test]
    #[traced_test]
    fn hard_bread() {
        let document = "> [!warning]  \n> Te";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_hard_break()).is_some());

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.kind(), CalloutKind::Warning);

        // Skip Hard
        assert_eq!(callout.title(), None);
        assert_eq!(
            callout.content().nth(1).unwrap().as_plain_text().unwrap(),
            "Te"
        );
    }

    #[test]
    fn title_with_text() {
        let document = "> [!tip] Important note\nContent";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Important note"));

        assert_eq!(callout.kind(), CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_trimmed() {
        let document = "> [!tip]   Spaced title   \nContent";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Spaced title"));

        assert_eq!(callout.kind(), CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_empty_after_trim() {
        let document = "> [!tip]   \nContent";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), None);

        assert_eq!(callout.kind(), CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_with_foldable() {
        let document = "> [!tip]+ Folded title\nContent";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Folded title"));

        assert_eq!(callout.kind(), CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::Expanded);
    }

    #[test]
    fn title_no_content() {
        let document = "> [!tip]";
        let ast = Parser::new(document).ast();

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), None);

        assert_eq!(callout.kind(), CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }
}
