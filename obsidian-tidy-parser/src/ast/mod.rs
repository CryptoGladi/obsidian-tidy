pub mod node;
pub mod stack;

use crate::token_stream::Token;
use node::{BlockQuote, Callout, Heading, Node, NodeKind, Paragraph, Root, Strong, Tag};
use pulldown_cmark::{Event as MarkEvent, Tag as MarkTag, TagEnd as MarkTagEnd};
use stack::{Frame, Stack};
use std::range::Range;
use tracing::instrument;

pub struct ASTBuilder<I> {
    inner: I,
}

impl<I> ASTBuilder<I> {
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<'ast, I> ASTBuilder<I>
where
    I: Iterator<Item = (Token<'ast>, Range<usize>)>,
{
    fn process_tag_end(tag_end: MarkTagEnd, stack: &mut Stack<'ast>, offset: Range<usize>) {
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
            MarkTag::Strong => {
                let strong = Strong::new();
                let tag = Tag::new(strong, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Strong(tag), offset));
            }
            MarkTag::BlockQuote(_) => {
                let block_quote = BlockQuote::new();
                let tag = Tag::new(block_quote, frame.children().into());

                stack.push_parent(Node::new(NodeKind::BlockQuote(tag), offset));
            }
            _ => todo!("{:?}", frame.tag),
        }

        debug_assert_eq!(tag_end, frame.tag.to_end());
    }

    fn process_token_markdown(
        markdown: MarkEvent<'ast>,
        stack: &mut Stack<'ast>,
        offset: Range<usize>,
    ) {
        match markdown {
            MarkEvent::Start(tag) => stack.push(Frame::new(tag, offset, Vec::new())),
            MarkEvent::End(tag_end) => Self::process_tag_end(tag_end, stack, offset),
            MarkEvent::Text(text) => {
                stack.push_parent(Node::new(NodeKind::Text(text.into()), offset));
            }
            MarkEvent::SoftBreak => stack.push_parent(Node::new(NodeKind::SoftBreak, offset)),
            MarkEvent::HardBreak => stack.push_parent(Node::new(NodeKind::HardBreak, offset)),
            MarkEvent::Code(text) => {
                stack.push_parent(Node::new(NodeKind::InlineCode(text.into()), offset));
            }
            _ => todo!("{:?}", markdown),
        }
    }

    #[expect(
        clippy::missing_panics_doc,
        reason = "каждая строка — это валидный Markdown"
    )]
    #[instrument(skip_all, level = "trace", fields(node_kind, child_count, parse_range))]
    pub fn build(self) -> Node<'ast> {
        let mut stack = Stack::new();

        for (token, offset) in self.inner {
            tracing::trace!(?token, ?offset, "building AST");

            match token {
                Token::Markdown(markdown) => {
                    Self::process_token_markdown(markdown, &mut stack, offset)
                }
                _ => todo!("{:?}", token),
            }
        }

        #[expect(
            clippy::expect_used,
            reason = "В конце на дне стека останутся все корневые узлы документа"
        )]
        let children_root = stack.into_root().expect("stack not empty");
        let root = children_root.into_boxed_slice();

        let offset_end = root.last().map_or(0, |node| node.offset().end);
        let root = Tag::new(Root::new(), root);
        let root = Node::new(NodeKind::Root(root), (0..offset_end).into());

        tracing::trace!(
            parse_range = ?root.offset(),
            node_count = root.node_count(),
            "AST build finished"
        );

        root
    }
}

pub trait ASTBuildExt<'ast>: Iterator {
    fn build_ast(self) -> Node<'ast>;
}

impl<'ast, I> ASTBuildExt<'ast> for I
where
    I: Iterator<Item = (Token<'ast>, Range<usize>)>,
{
    fn build_ast(self) -> Node<'ast> {
        ASTBuilder::new(self).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{markdown_lexer::MarkdownLexerBuilder, token_stream::TokenStream};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn empty() {
        let document = "";
        let lexer = MarkdownLexerBuilder::default().build(document);
        let token_stream = TokenStream::new(document, lexer, []);

        let ast = token_stream.build_ast();

        let children = ast.as_root().unwrap().children();
        assert!(children.is_empty());
    }
}
