use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn collect_map<F, B, T>(&'ast self, predicate: F) -> B
    where
        B: Default + Extend<T>,
        F: FnMut(&'ast Node<'ast>) -> Option<T>,
    {
        let mut predicate = predicate;
        let mut acc = B::default();

        self.for_each(|node| {
            if let Some(data) = predicate(node) {
                acc.extend(core::iter::once(data));
            }
        });

        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, TextContent};
    use std::collections::HashMap;

    #[test]
    fn collect_map_strong() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let strongs: Vec<_> = ast.collect_map(|node| node.as_strong());
        let text_strongs: Vec<_> = strongs
            .into_iter()
            .map(|strong| strong.as_plain_text().unwrap())
            .collect();

        assert_eq!(text_strongs, ["super", "Rust"]);
    }

    #[test]
    fn collect_map_use_mut_fn() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut var_mut = 0;

        let strongs: Vec<_> = ast.collect_map(|node| {
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
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let headings: Vec<_> = ast.collect_map(|node| node.as_heading());
        assert!(headings.is_empty());
    }

    #[test]
    fn collect_hash_map() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut idx = 0usize;
        let map: HashMap<_, _> = ast.collect_map(|node| {
            idx += 1;
            if let Some(text) = node.as_text() {
                return Some((idx, text));
            }

            None
        });

        assert_eq!(map.len(), 5);
    }
}
