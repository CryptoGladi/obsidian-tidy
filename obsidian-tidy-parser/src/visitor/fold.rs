use super::Visitor;
use crate::prelude::Node;

pub trait Fold {
    type Output;

    fn finish(self) -> Self::Output;
}

pub trait FoldVisitorExt<'a> {
    fn fold_visitor<V>(&'a self, visitor: V) -> V::Output
    where
        V: Visitor<'a> + Fold;
}

impl<'a> FoldVisitorExt<'a> for Node<'a> {
    fn fold_visitor<V>(&'a self, visitor: V) -> V::Output
    where
        V: Visitor<'a> + Fold,
    {
        let mut visitor = visitor;
        let _ = visitor.visit_node(self);

        visitor.finish()
    }
}

// Там проверка одного случая! Все остальные
// тесты находся в node!
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::{borrow::Cow, ops::ControlFlow, range::Range};

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

    impl Fold for CountWord {
        type Output = usize;

        fn finish(self) -> Self::Output {
            self.count
        }
    }

    #[test]
    fn visit_node() {
        let document = "# Hello world everyone!";
        let ast = Parser::new(&document).ast();

        let count_word = CountWord::default();
        let result = ast.fold_visitor(count_word);

        assert_eq!(result, 3);
    }
}
