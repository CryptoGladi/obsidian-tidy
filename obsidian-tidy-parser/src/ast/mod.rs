pub mod node;
pub mod stack;

use crate::token_stream::token::{Tag as TokenTag, TagEnd, Token};
use alloc::vec::Vec;
use core::range::Range;
use node::{BlockQuote, Callout, Node, NodeKind, Paragraph, Root, Strong, Tag};
use stack::{Frame, Stack};
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
    fn process_tag_end(tag_end: TagEnd, stack: &mut Stack<'ast>, offset: Range<usize>) {
        #[expect(
            clippy::expect_used,
            reason = "библиотека гарантирует, что это невозможно"
        )]
        let frame = stack.pop().expect("unbalanced tags");

        debug_assert_eq!(tag_end, frame.tag.to_end());

        match frame.tag {
            TokenTag::Heading(heading) => {
                let tag = Tag::new(heading, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Heading(tag), offset));
            }
            TokenTag::Paragraph => {
                let paragraph = Paragraph::new();
                let tag = Tag::new(paragraph, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Paragraph(tag), offset));
            }
            TokenTag::Strong => {
                let strong = Strong::new();
                let tag = Tag::new(strong, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Strong(tag), offset));
            }
            TokenTag::BlockQuote => {
                let block_quote = BlockQuote::new();
                let tag = Tag::new(block_quote, frame.children().into());

                stack.push_parent(Node::new(NodeKind::BlockQuote(tag), offset));
            }
            TokenTag::Callout(ref callout) => {
                let callout = Callout::new(
                    callout.kind.clone(),
                    callout.header_offset,
                    callout.foldable,
                );
                let tag = Tag::new(callout, frame.children().into());

                stack.push_parent(Node::new(NodeKind::Callout(tag), offset));
            }
            _ => todo!("{:?}", frame.tag),
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
                Token::Start(tag) => stack.push(Frame::new(tag, offset, Vec::new())),
                Token::End(tag_end) => Self::process_tag_end(tag_end, &mut stack, offset),
                Token::Text(text) => {
                    stack.push_parent(Node::new(NodeKind::Text(text), offset));
                }
                Token::SoftBreak => stack.push_parent(Node::new(NodeKind::SoftBreak, offset)),
                Token::HardBreak => stack.push_parent(Node::new(NodeKind::HardBreak, offset)),
                Token::Code(text) => {
                    stack.push_parent(Node::new(NodeKind::InlineCode(text), offset));
                }
                Token::Rule => stack.push_parent(Node::new(NodeKind::Rule, offset)),
                Token::InlineMath(text) => {
                    stack.push_parent(Node::new(NodeKind::InlineMath(text), offset));
                }
                Token::DisplayMath(text) => {
                    stack.push_parent(Node::new(NodeKind::DisplayMath(text), offset));
                }
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
    use crate::{
        markdown_lexer::MarkdownLexerBuilder,
        token_stream::{TokenStream, interceptor::InterceptorEnum},
    };

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn empty() {
        let document = "";
        let lexer = MarkdownLexerBuilder::default().build(document);
        let token_stream = TokenStream::<InterceptorEnum>::new(document, lexer, []);

        let ast = token_stream.build_ast();

        let children = ast.as_root().unwrap().children();
        assert!(children.is_empty());
    }
}
