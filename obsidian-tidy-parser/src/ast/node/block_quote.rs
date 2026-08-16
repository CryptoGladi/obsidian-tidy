use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BlockQuote;

impl BlockQuote {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

super::impl_node_as!(BlockQuote);

#[cfg(test)]
mod tests {
    use crate::prelude::{Parser, TextContent};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn ast() {
        let document = "# Define\n>My **super** quote";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[traced_test]
    fn nested_ast() {
        let document = "# Define\n>My **super** quote\n>> Quote by quote";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[traced_test]
    fn as_plain_text() {
        let document = "# Define\n> My quote";
        let ast = Parser::new(document).ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert_eq!(quote.as_plain_text().unwrap(), "My quote");
    }

    #[test]
    #[traced_test]
    fn as_plain_text_with_formatting() {
        let document = "# Define\n> My **super** quote";
        let ast = Parser::new(document).ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert!(quote.as_plain_text().is_none());
    }
}
