mod fold;
mod macros;

pub use fold::{Fold, FoldVisitorExt};

use crate::prelude::{BlockQuote, Callout, Heading, Node, Paragraph, Root, Strong};
use macros::define_visitor;
use std::borrow::Cow;

define_visitor! {
    tagged {
        Root: Root,
        Paragraph: Paragraph,
        Heading: Heading,
        Strong: Strong,
        BlockQuote: BlockQuote,
        Callout: Callout
    }
    leaf {
        Text: Cow<'ast, str>,
        InlineCode: Cow<'ast, str>
    }
    empty {
        SoftBreak,
        HardBreak
    }
}

pub trait VisitExt<'a> {
    fn visit<V>(&'a self, visitor: &mut V)
    where
        V: Visitor<'a>;
}

impl<'a> VisitExt<'a> for Node<'a> {
    fn visit<V>(&'a self, visitor: &mut V)
    where
        V: Visitor<'a>,
    {
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
        fn visit_text(&mut self, text: &Cow<'_, str>, _offset: Range<usize>) -> ControlFlow<()> {
            self.count += text.split_whitespace().count();
            ControlFlow::Continue(())
        }
    }

    #[test]
    fn visit_node() {
        let document = "# Hello world everyone!";
        let ast = Parser::new(&document).ast();

        let mut count_word = CountWord::default();
        let _ = count_word.visit_node(&ast);

        assert_eq!(count_word.count, 3);
    }

    #[test]
    fn visit_ext() {
        let document = "# Hello world everyone!";
        let ast = Parser::new(&document).ast();

        let mut count_word = CountWord::default();
        ast.visit(&mut count_word);

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

        let document = "# Hello world everyone!";
        let ast = Parser::new(&document).ast();

        let mut call_order = CallOrderVisitor::new();
        ast.visit(&mut call_order);

        assert_eq!(
            call_order.calls,
            ["pre_visit_heading", "visit_heading", "post_visit_heading"]
        );
    }
}
