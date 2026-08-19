use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Root;

impl Root {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

super::impl_node_as!(Root);

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, TextContent};

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn as_plain_text() {
        let document = Document::new("Simple text");
        let ast = document.ast();

        assert_eq!(ast.as_plain_text().unwrap(), "Simple text");
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn as_plain_text_with_formatting() {
        let document = Document::new("My **super** text");
        let ast = document.ast();

        assert!(ast.as_plain_text().is_none());
    }
}
