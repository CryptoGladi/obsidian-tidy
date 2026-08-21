mod kind;

use crate::{
    prelude::{Node, NodeKind, Tag},
    token_stream::token::CalloutFoldable,
};
use alloc::borrow::Cow;
use alloc::string::ToString;
use core::range::Range;
pub use kind::CalloutKind;
use serde::{Deserialize, Serialize};

const EXPECT_MESSAGE: &str = "Callout should have validated structure";

pub struct CalloutContentIter<'ast> {
    iter: core::slice::Iter<'ast, Node<'ast>>,
    skip_first_char: bool,
}

impl<'ast> CalloutContentIter<'ast> {
    const fn new(iter: core::slice::Iter<'ast, Node<'ast>>, skip_first_char: bool) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Callout<'ast> {
    pub foldable: CalloutFoldable,
    pub kind: CalloutKind<'ast>,

    #[serde(with = "crate::__private::range_serde")]
    pub header_offset: Range<usize>,
}

impl<'ast> Callout<'ast> {
    #[must_use]
    pub fn new(
        kind: Cow<'ast, str>,
        header_offset: Range<usize>,
        foldable: CalloutFoldable,
    ) -> Self {
        Self {
            foldable,
            kind: CalloutKind::from(kind),
            header_offset,
        }
    }
}

impl<'ast> From<&crate::token_stream::token::Callout<'ast>> for Callout<'ast> {
    fn from(callout: &crate::token_stream::token::Callout<'ast>) -> Self {
        // It is Cow!
        // Clone is fast!
        Self::new(
            callout.kind.clone(),
            callout.header_offset,
            callout.foldable,
        )
    }
}

impl<'ast> Tag<'ast, Callout<'ast>> {
    #[expect(
        clippy::expect_used,
        clippy::missing_panics_doc,
        reason = "Callout::parse should have validated structure"
    )]
    #[must_use]
    pub fn content(&'ast self) -> CalloutContentIter<'ast> {
        let first_child = self.children().first().expect(EXPECT_MESSAGE);
        let paragraph = first_child.as_paragraph().expect(EXPECT_MESSAGE);
        let mut paragraph_iter = paragraph.children().iter();

        // Skip Text("[")
        // Skip Text("!tip")
        // Skip Text("]")
        paragraph_iter.nth(2);

        let skip_first_char = !self.kind.foldable.is_none();
        CalloutContentIter::new(paragraph_iter, skip_first_char)
    }

    #[must_use]
    pub const fn callout_kind(&self) -> &CalloutKind<'ast> {
        &self.kind.kind
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
        let first_child = self.children().first().expect(EXPECT_MESSAGE);
        let paragraph = first_child.as_paragraph().expect(EXPECT_MESSAGE);
        let children = paragraph.children();

        // Пропускаем Text[, !type, ]
        let content_start = 3;
        if children.len() <= content_start {
            return None;
        }

        let first_content = &children[content_start];
        let text = first_content.as_text()?;

        let text = if self.kind.foldable.is_none() {
            text
        } else {
            text.strip_prefix(['+', '-'])?
        };

        let trimmed = text.trim();
        Some(trimmed)
    }
}

super::impl_node_as!(Callout, Callout<'_>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{ASTBuildExt, Node, TextContent, TokenStreamBuilder};

    fn get_ast(source: &str) -> Node<'_> {
        TokenStreamBuilder::default().build(source).build_ast()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn without_title() {
        let source = "> [!tip]\nText";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);

        // Skip SoftBreak
        assert_eq!(callout.title(), None);
        assert_eq!(
            callout.content().nth(1).unwrap().as_plain_text().unwrap(),
            "Text"
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn many_space() {
        let source = ">  [!tip] Text";
        let ast = get_ast(source);

        assert!(ast.find_map(|node| node.as_callout()).is_none());
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn zero_space() {
        let source = ">[!example]+ Text\n> Other Data";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::Expanded);
        assert_eq!(callout.callout_kind(), &CalloutKind::Example);

        assert_eq!(callout.title().unwrap(), "Text");
        assert_eq!(
            callout.content().next().unwrap().as_plain_text().unwrap(),
            " Text"
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn without_text() {
        let source = "> [!warning]";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.callout_kind(), &CalloutKind::Warning);

        assert_eq!(callout.title(), None);
        assert_eq!(callout.content().next(), None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn hard_bread() {
        let source = "> [!warning]  \n> Te";
        let ast = get_ast(source);

        assert!(ast.find(|node| node.kind().is_hard_break()).is_some());

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.foldable(), CalloutFoldable::None);
        assert_eq!(callout.callout_kind(), &CalloutKind::Warning);

        // Skip Hard
        assert_eq!(callout.title(), None);
        assert_eq!(
            callout.content().nth(1).unwrap().as_plain_text().unwrap(),
            "Te"
        );
    }

    #[test]
    fn title_with_text() {
        let source = "> [!tip] Important note\nContent";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Important note"));

        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_trimmed() {
        let source = "> [!tip]   Spaced title   \nContent";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Spaced title"));

        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_empty_after_trim() {
        let source = "> [!tip]   \nContent";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), None);

        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }

    #[test]
    fn title_with_foldable() {
        let source = "> [!tip]+ Folded title\nContent";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), Some("Folded title"));

        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::Expanded);
    }

    #[test]
    fn title_no_content() {
        let source = "> [!tip]";
        let ast = get_ast(source);

        let callout = ast.find_map(|node| node.as_callout()).unwrap();
        assert_eq!(callout.title(), None);

        assert_eq!(callout.callout_kind(), &CalloutKind::Tip);
        assert_eq!(callout.foldable(), CalloutFoldable::None);
    }
}
