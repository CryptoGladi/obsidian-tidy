use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
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
    use crate::{TextContent, prelude::Parser};
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn check_have_paragraph() {
        let document = "Super text";
        let ast = Parser::new(document).ast();

        assert!(ast.find(|node| node.kind().is_paragraph()).is_some());
        insta::assert_json_snapshot!(ast);
    }

    #[test]
    #[traced_test]
    fn as_plain_text() {
        let document = "# Heading\nSimple text";
        let ast = Parser::new(document).ast();

        let paragraph = ast.find_map(|node| node.as_paragraph()).unwrap();
        assert_eq!(paragraph.as_plain_text().unwrap(), "Simple text");
    }
}
