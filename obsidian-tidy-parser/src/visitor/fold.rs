use super::Visitor;
use crate::prelude::Node;

pub trait FoldVisitor<'a>: Visitor<'a> {
    type Output;

    fn finish(self) -> Self::Output;
}

pub trait FoldVisitorExt<'a> {
    fn fold<V>(&self, visitor: V) -> V::Output
    where
        V: FoldVisitor<'a>;
}

impl<'a> FoldVisitorExt<'a> for Node<'a> {
    fn fold<V>(&self, visitor: V) -> V::Output
    where
        V: FoldVisitor<'a>,
    {
        let mut visitor = visitor;
        visitor.visit_node(self);

        visitor.finish()
    }
}

// Там проверка одного случая! Все остальные
// тесты находся в node!
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::range::Range;

    #[derive(Debug, Default)]
    struct CountWord {
        count: usize,
    }

    impl Visitor<'_> for CountWord {
        fn visit_text(&mut self, text: &CowStr<'_>, _offset: Range<usize>) {
            self.count += text.split_whitespace().count();
        }
    }

    impl FoldVisitor<'_> for CountWord {
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
        let result = ast.fold(count_word);

        assert_eq!(result, 3);
    }
}
