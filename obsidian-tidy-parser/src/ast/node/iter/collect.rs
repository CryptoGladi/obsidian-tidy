use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn collect<F>(&'ast self, predicate: F) -> Vec<&'ast Node<'ast>>
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;
        let mut acc = Vec::new();

        self.for_each(|node| {
            if predicate(node) {
                acc.push(node);
            }
        });

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Parser, TextContent};

    #[test]
    fn collect_strong() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let strongs = ast.collect(|node| node.kind().is_strong());
        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|node| node.as_strong().unwrap().as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
    }

    #[test]
    fn collect_use_mut_fn() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let mut var_mut = 0;

        let strongs = ast.collect(|node| {
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
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let headings = ast.collect(|node| node.kind().is_heading());
        assert!(headings.is_empty());
    }
}
