use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn collect<B, F>(&'ast self, predicate: F) -> B
    where
        B: Default + Extend<&'ast Node<'ast>>,
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;
        let mut acc = B::default();

        self.for_each(|node| {
            if predicate(node) {
                acc.extend(std::iter::once(node));
            }
        });

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, TextContent};

    #[test]
    fn collect_strong() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let strongs: Vec<_> = ast.collect(|node| node.kind().is_strong());
        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|node| node.as_strong().unwrap().as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
    }

    #[test]
    fn collect_use_mut_fn() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut var_mut = 0;

        let strongs: Vec<_> = ast.collect(|node| {
            var_mut += 1;
            node.kind().is_strong()
        });

        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|node| node.as_strong().unwrap().as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
        assert_ne!(var_mut, 0);
    }

    #[test]
    fn collect_empty() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let headings: Vec<_> = ast.collect(|node| node.kind().is_heading());
        assert!(headings.is_empty());
    }
}
