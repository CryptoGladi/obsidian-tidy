pub mod node;
pub mod stack;

use alloc::vec::Vec;
use core::range::Range;
use node::{
    BlockQuote, Callout, CodeBlock, Emphasis, HtmlBlock, Item, List, Node, NodeKind, Paragraph,
    Root, Strong, Tag,
};
use obsidian_tidy_lexer::{Tag as TokenTag, TagEnd, Token};
use stack::{Frame, Stack};

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
        let children = frame.children().into();

        debug_assert_eq!(tag_end, frame.tag.to_end());

        // TODO написать макрос
        match frame.tag {
            TokenTag::Heading(heading) => {
                let tag = Tag::new(heading, children);

                stack.push_parent(Node::new(NodeKind::Heading(tag), offset));
            }
            TokenTag::Paragraph => {
                let paragraph = Paragraph::new();
                let tag = Tag::new(paragraph, children);

                stack.push_parent(Node::new(NodeKind::Paragraph(tag), offset));
            }
            TokenTag::Strong => {
                let strong = Strong::new();
                let tag = Tag::new(strong, children);

                stack.push_parent(Node::new(NodeKind::Strong(tag), offset));
            }
            TokenTag::BlockQuote => {
                let block_quote = BlockQuote::new();
                let tag = Tag::new(block_quote, children);

                stack.push_parent(Node::new(NodeKind::BlockQuote(tag), offset));
            }
            TokenTag::Callout(callout) => {
                let callout = Callout::from(callout);
                let tag = Tag::new(callout, children);

                stack.push_parent(Node::new(NodeKind::Callout(tag), offset));
            }
            TokenTag::CodeBlock(code_block) => {
                let code_block = CodeBlock::from(code_block);
                let tag = Tag::new(code_block, children);

                stack.push_parent(Node::new(NodeKind::CodeBlock(tag), offset));
            }
            TokenTag::Emphasis => {
                let emphasis = Emphasis;
                let tag = Tag::new(emphasis, children);

                stack.push_parent(Node::new(NodeKind::Emphasis(tag), offset));
            }
            TokenTag::HtmlBlock => {
                let html_block = HtmlBlock;
                let tag = Tag::new(html_block, children);

                stack.push_parent(Node::new(NodeKind::HtmlBlock(tag), offset));
            }
            TokenTag::List(list) => {
                let list = List::from(list);
                let tag = Tag::new(list, children);

                stack.push_parent(Node::new(NodeKind::List(tag), offset));
            }
            TokenTag::Item => {
                let item = Item;
                let tag = Tag::new(item, children);

                stack.push_parent(Node::new(NodeKind::Item(tag), offset));
            }
            _ => {
                debug_assert!(false, "UNSOPPORT");
            }
        }
    }

    #[expect(
        clippy::missing_panics_doc,
        reason = "каждая строка — это валидный Markdown"
    )]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, level = "trace"))]
    pub fn build(self) -> Node<'ast> {
        let mut stack = Stack::new();

        for (token, offset) in self.inner {
            #[cfg(feature = "tracing")]
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
                _ => {
                    debug_assert!(false, "UNSOPPORT");
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

        #[cfg(feature = "tracing")]
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
    use obsidian_tidy_lexer::{InterceptorEnum, TokenStream};

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
