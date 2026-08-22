use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn all<F>(&'ast self, predicate: F) -> bool
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;

        // Test optimization: use `dyn FnMut` in an inner function
        // to erase the closure type and prevent code bloat.
        self.all_inner(&mut predicate)
    }

    #[cfg(debug_assertions)]
    #[doc(hidden)]
    #[inline(never)]
    fn all_inner(&'ast self, predicate: &mut dyn FnMut(&'ast Node<'ast>) -> bool) -> bool {
        self.find(|node| !predicate(node)).is_none()
    }

    #[cfg(not(debug_assertions))]
    #[doc(hidden)]
    #[inline]
    fn all_inner<F>(&'ast self, mut predicate: F) -> bool
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        self.find(|node| !predicate(node)).is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, NodeKind};

    #[test]
    fn all_positive() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        assert!(ast.all(|node| {
            match node.kind() {
                NodeKind::Root(_)
                | NodeKind::Text(_)
                | NodeKind::Strong(_)
                | NodeKind::Paragraph(_)
                | NodeKind::InlineCode(_) => true,
                _ => false,
            }
        }));
    }

    #[test]
    fn all_negative() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        assert!(!ast.all(|node| node.kind().is_heading()));
    }

    #[test]
    fn any_use_mut_fn() {
        let text = "My **super** document with `code` and **Rust**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut var_mut = 0;

        let have_strong = ast.any(|node| {
            var_mut += 1;

            node.kind().is_strong()
        });

        assert!(have_strong);
        assert_ne!(var_mut, 0);
    }
}
