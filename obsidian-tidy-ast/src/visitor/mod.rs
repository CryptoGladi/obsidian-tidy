mod fold;
mod macros;

pub use fold::{Fold, FoldVisitorExt};

use crate::prelude::{
    BlockQuote, CodeBlock, Emphasis, Heading, HtmlBlock, Item, List, Node, Paragraph, Root, Strong,
};
use macros::define_visitor;

define_visitor! {
    tagged {
        Root: Root,
        Paragraph: Paragraph,
        Heading: Heading,
        Strong: Strong,
        Emphasis: Emphasis,
        BlockQuote: BlockQuote,
        //Callout: Callout,
        CodeBlock: CodeBlock,
        HtmlBlock: HtmlBlock,
        List: List,
        Item: Item
    }
    leaf {
        Text: str,
        InlineCode: str,

        InlineMath: str,
        DisplayMath: str
    }
    empty {
        SoftBreak,
        HardBreak,
        Rule
    }
}

static_assertions::assert_obj_safe!(Visitor);

#[cfg(not(debug_assertions))]
pub trait VisitExt<'ast> {
    fn visit<V>(&'ast self, visitor: &mut V)
    where
        V: Visitor<'ast> + ?Sized;
}

#[cfg(not(debug_assertions))]
impl<'ast> VisitExt<'ast> for Node<'ast> {
    #[inline]
    fn visit<V>(&'ast self, visitor: &mut V)
    where
        V: Visitor<'ast> + ?Sized,
    {
        let _ = visitor.visit_node(self);
    }
}

/// Test optimization: switch to `dyn Visitor` (dynamic dispatch)
/// to prevent monomorphization of a large trait and reduce compile times.
#[cfg(debug_assertions)]
pub trait VisitExt<'ast> {
    fn visit(&'ast self, visitor: &mut dyn Visitor<'ast>);
}

#[cfg(debug_assertions)]
impl<'ast> VisitExt<'ast> for Node<'ast> {
    #[inline(never)]
    fn visit(&'ast self, visitor: &mut dyn Visitor<'ast>) {
        let _ = visitor.visit_node(self);
    }
}

// Там проверка одного случая! Все остальные
// тесты находся в node!
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{Tag, *};
    use std::{ops::ControlFlow, range::Range};

    #[derive(Debug, Default)]
    struct CountWord {
        count: usize,
    }

    impl Visitor<'_> for CountWord {
        fn visit_text(&mut self, text: &str, _offset: Range<usize>) -> ControlFlow<()> {
            self.count += text.split_whitespace().count();
            ControlFlow::Continue(())
        }
    }

    #[test]
    fn visit_node() {
        let document = Document::new("# Hello world everyone!");
        let ast = document.ast();

        let mut count_word = CountWord::default();
        let _ = count_word.visit_node(&ast);

        assert_eq!(count_word.count, 3);
    }

    #[test]
    fn visit_ext() {
        let document = Document::new("# Hello world everyone!");
        let ast = document.ast();

        let mut count_word = CountWord::default();
        ast.visit(&mut count_word);

        assert_eq!(count_word.count, 3);
    }

    #[test]
    fn visit_ext_not_sized() {
        let document = Document::new("# Hello world everyone!");
        let ast = document.ast();

        let mut count_word = CountWord::default();
        let dyn_count_word: &mut dyn Visitor = &mut count_word;

        ast.visit(dyn_count_word);
        assert_eq!(count_word.count, 3);
    }

    #[test]
    fn call_order_pre_post() {
        struct CallOrderVisitor {
            calls: Vec<String>,
        }

        impl CallOrderVisitor {
            fn new() -> Self {
                Self { calls: Vec::new() }
            }

            fn push(&mut self, message: impl Into<String>) {
                self.calls.push(message.into());
            }
        }

        impl Visitor<'_> for CallOrderVisitor {
            fn pre_visit_heading(
                &mut self,
                _tag: &Tag<'_, Heading>,
                _offset: Range<usize>,
            ) -> ControlFlow<()> {
                self.push("pre_visit_heading");
                ControlFlow::Continue(())
            }

            fn post_visit_heading(&mut self, _tag: &Tag<'_, Heading>, _offset: Range<usize>) {
                self.push("post_visit_heading");
            }

            fn visit_heading(
                &mut self,
                tag: &Tag<'_, Heading>,
                offset: Range<usize>,
            ) -> ControlFlow<()> {
                self.pre_visit_heading(tag, offset)?;

                self.push("visit_heading");
                for child in tag.children() {
                    self.visit_node(child)?;
                }

                self.post_visit_heading(tag, offset);
                ControlFlow::Continue(())
            }
        }

        let document = Document::new("# Hello world everyone!");
        let ast = document.ast();

        let mut call_order = CallOrderVisitor::new();
        ast.visit(&mut call_order);

        assert_eq!(
            call_order.calls,
            ["pre_visit_heading", "visit_heading", "post_visit_heading"]
        );
    }
}
