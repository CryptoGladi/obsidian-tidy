use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn count<F>(&'ast self, predicate: F) -> usize
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;

        self.fold(
            0usize,
            |acc, node| if predicate(node) { acc + 1 } else { acc },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Document;

    #[test]
    fn counting_use_strong() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let count_strong = ast.count(|node| node.kind().is_strong());
        assert_eq!(count_strong, 2);
    }

    #[test]
    fn counting_use_mut_fn() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut mut_var = 0;
        let count_strong = ast.count(|node| {
            mut_var += 1;

            node.kind().is_strong()
        });

        assert_eq!(count_strong, 2);
        assert_ne!(mut_var, 0)
    }

    #[test]
    fn counting_but_not_found() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let count_strong = ast.count(|node| node.kind().is_heading());
        assert_eq!(count_strong, 0);
    }
}
