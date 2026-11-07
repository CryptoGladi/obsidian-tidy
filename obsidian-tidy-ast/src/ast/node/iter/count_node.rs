use crate::prelude::Node;

impl Node<'_> {
    #[inline]
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.count(|_| true)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Document;

    #[test]
    fn count_empty() {
        let text = "";
        let document = Document::new(text);
        let ast = document.ast();

        // Root only
        assert_eq!(ast.node_count(), 1);
    }

    #[test]
    fn count_text() {
        let text = "# Define\n**Super** text";
        let document = Document::new(text);
        let ast = document.ast();

        // Root + Heading + Text + SoftBreak + (Storng + Text) + Text = 7
        assert_eq!(ast.node_count(), 7);
    }
}
