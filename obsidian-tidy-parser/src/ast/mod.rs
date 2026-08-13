pub mod node;
pub mod stack;

use node::{Heading, Node, NodeKind, Paragraph, Root, Tag};
use pulldown_cmark::{Event as MarkEvent, Tag as MarkTag, TagEnd as MarkTagEnd};
use stack::{Frame, Stack};
use std::range::Range;

pub struct ASTBuilder<I> {
    inner: I,
}

impl<I> ASTBuilder<I> {
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<'a, I> ASTBuilder<I>
where
    I: Iterator<Item = (MarkEvent<'a>, Range<usize>)>,
{
    fn process_tag_end(tag_end: MarkTagEnd, stack: &mut Stack<'a>, offset: Range<usize>) {
        #[expect(
            clippy::expect_used,
            reason = "библиотека гарантирует, что это невозможно"
        )]
        let frame = stack.pop().expect("unbalanced tags");

        match frame.tag {
            MarkTag::Heading { level, .. } => {
                let heading = Heading::new(level.into());
                let tag = Tag::new(heading, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Heading(tag), offset));
            }
            MarkTag::Paragraph => {
                let paragraph = Paragraph::new();
                let tag = Tag::new(paragraph, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Paragraph(tag), offset));
            }
            _ => todo!(),
        }

        debug_assert_eq!(tag_end, frame.tag.to_end());
    }

    #[expect(
        clippy::missing_panics_doc,
        reason = "каждая строка — это валидный Markdown"
    )]
    pub fn build(self) -> Node<'a> {
        let mut stack = Stack::new();

        for (event, offset) in self.inner {
            match event {
                MarkEvent::Start(tag) => stack.push(Frame::new(tag, Vec::new())),
                MarkEvent::End(tag_end) => {
                    Self::process_tag_end(tag_end, &mut stack, offset);
                }
                MarkEvent::Text(text) => {
                    stack.push_parent(Node::new(NodeKind::Text(text), offset));
                }
                MarkEvent::SoftBreak => {
                    stack.push_parent(Node::new(NodeKind::SoftBreak, offset));
                }
                _ => todo!(),
            }
        }

        #[expect(
            clippy::expect_used,
            reason = "В конце на дне стека останутся все корневые узлы документа"
        )]
        let children_root = stack.into_root().expect("stack not empty");
        let children_root = children_root.into_boxed_slice();

        let offset_end = children_root.last().map_or(0, |node| node.offset().end);

        let root = Tag::new(Root::new(), children_root);
        Node::new(NodeKind::Root(root), (0..offset_end).into())
    }
}

pub trait ASTBuildExt<'a>: Iterator {
    fn build_ast(self) -> Node<'a>;
}

impl<'a, I> ASTBuildExt<'a> for I
where
    I: Iterator<Item = (MarkEvent<'a>, Range<usize>)>,
{
    fn build_ast(self) -> Node<'a> {
        ASTBuilder::new(self).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::Parser as MarkParser;

    #[test]
    fn empty() {
        let document = "";
        let ast = MarkParser::new(document)
            .into_offset_iter()
            .map(|(event, range)| (event, std::range::Range::from(range)))
            .build_ast();

        let children = ast.as_root().unwrap().children();
        assert!(children.is_empty());
    }
}
