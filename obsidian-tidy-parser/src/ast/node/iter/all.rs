use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn all<F>(&'ast self, predicate: F) -> bool
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;
        self.find(|node| !predicate(node)).is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::NodeKind;
    use crate::prelude::Parser;

    #[test]
    fn all_positive() {
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

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
        let document = "My **super** document with `code` and **Rust**";
        let ast = Parser::new(document).ast();

        assert!(!ast.all(|node| node.kind().is_heading()));
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
