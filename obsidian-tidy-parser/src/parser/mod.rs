mod builder;

use crate::{ast::ASTBuildExt, prelude::Node};
use pulldown_cmark::OffsetIter;

pub use builder::ParserBuilder;

#[derive(Debug)]
pub struct Parser<'input> {
    inner: OffsetIter<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(text: &'input str) -> Self {
        ParserBuilder::default().build(text)
    }

    pub fn ast(self) -> Node<'input> {
        self.inner
            .map(|(event, offset)| (event, std::range::Range::from(offset)))
            .build_ast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast() {
        let document = "# Heading\nLol";
        let ast = Parser::new(document).ast();

        insta::assert_json_snapshot!(ast);
    }
}
