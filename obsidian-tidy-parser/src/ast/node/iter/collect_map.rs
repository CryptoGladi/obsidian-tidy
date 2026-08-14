use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn collect_map<F, T>(&'ast self, predicate: F) -> Vec<T>
    where
        F: FnMut(&'ast Node<'ast>) -> Option<T>,
    {
        let mut predicate = predicate;
        let mut acc = Vec::new();

        self.for_each(|node| {
            if let Some(data) = predicate(node) {
                acc.push(data);
            }
        });

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Parser, TextContent};

    #[test]
    fn collect_map_strong() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let strongs = ast.collect_map(|node| node.as_strong());
        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|strong| strong.as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
    }

    #[test]
    fn collect_map_use_mut_fn() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let mut var_mut = 0;

        let strongs = ast.collect_map(|node| {
            var_mut += 1;
            node.as_strong()
        });

        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|node| node.as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
        assert_ne!(var_mut, 0);
    }

    #[test]
    fn collect_map_empty() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        let headings = ast.collect_map(|node| node.as_heading());
        assert!(headings.is_empty());
    }
}
