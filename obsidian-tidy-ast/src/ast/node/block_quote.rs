use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    use crate::prelude::{Document, TextContent};

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn ast() {
        let text = "# Define\n>My **super** quote";
        let document = Document::new(text);
        let ast = document.ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn nested_ast() {
        let text = "# Define\n>My **super** quote\n>> Quote by quote";
        let document = Document::new(text);
        let ast = document.ast();

        assert!(ast.find(|node| node.kind().is_block_quote()).is_some());

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn as_plain_text() {
        let text = "# Define\n> My quote";
        let document = Document::new(text);
        let ast = document.ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert_eq!(quote.as_plain_text().unwrap(), "My quote");
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn as_plain_text_with_formatting() {
        let text = "# Define\n> My **super** quote";
        let document = Document::new(text);
        let ast = document.ast();

        let quote = ast.find_map(|node| node.as_block_quote()).unwrap();
        assert!(quote.as_plain_text().is_none());
    }
}
