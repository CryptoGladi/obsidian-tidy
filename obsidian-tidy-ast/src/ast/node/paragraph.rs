use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paragraph;

impl Paragraph {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

super::impl_node_as!(Paragraph);

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, TextContent};

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn check_have_paragraph() {
        let document = Document::new("Super text");
        let ast = document.ast();

        assert!(ast.find(|node| node.kind().is_paragraph()).is_some());

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn as_plain_text() {
        let document = Document::new("# Heading\nSimple text");
        let ast = document.ast();

        let paragraph = ast.find_map(|node| node.as_paragraph()).unwrap();
        assert_eq!(paragraph.as_plain_text().unwrap(), "Simple text");
    }
}
