use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn any<F>(&'ast self, predicate: F) -> bool
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        self.find(predicate).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Parser;

    #[test]
    fn any_positive() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        assert!(ast.any(|node| node.kind().is_strong()));
    }

    #[test]
    fn any_negative() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        assert!(!ast.any(|node| node.kind().is_heading()));
    }

    #[test]
    fn any_use_mut_fn() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let mut var_mut = 0;

        let have_strong = ast.any(|node| {
            var_mut += 1;

            node.kind().is_strong()
        });

        assert!(have_strong);
        assert_ne!(var_mut, 0);
    }
}
