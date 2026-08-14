use crate::prelude::Node;

impl Node<'_> {
    #[inline]
    pub fn node_count(&self) -> usize {
        self.count(|_| true)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Parser;

    #[test]
    fn count_empty() {
        let document = "";
        let ast = Parser::new(document).ast();

        // Root only
        assert_eq!(ast.node_count(), 1);
    }

    #[test]
    fn count_text() {
        let document = "# Define\n**Super** text";
        let ast = Parser::new(document).ast();

        // Root + Heading + Text + SoftBreak + (Storng + Text) + Text = 7
        assert_eq!(ast.node_count(), 7);
    }
}
