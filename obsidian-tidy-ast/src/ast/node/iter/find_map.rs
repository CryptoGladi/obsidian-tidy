use crate::prelude::Node;
use core::ops::ControlFlow;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn find_map<F, T>(&'ast self, predicate: F) -> Option<T>
    where
        F: FnMut(&'ast Node<'ast>) -> Option<T>,
    {
        let mut predicate = predicate;

        // Test optimization: use `dyn FnMut` in an inner function
        // to erase the closure type and prevent code bloat.
        self.find_map_inner(&mut predicate)
    }

    #[cfg(debug_assertions)]
    #[doc(hidden)]
    #[inline(never)]
    fn find_map_inner<T>(
        &'ast self,
        predicate: &mut dyn FnMut(&'ast Node<'ast>) -> Option<T>,
    ) -> Option<T> {
        self.fold_while(None, |acc, node| {
            debug_assert!(acc.is_none());

            match predicate(node) {
                Some(target) => ControlFlow::Break(Some(target)),
                None => ControlFlow::Continue(None),
            }
        })
    }

    #[cfg(not(debug_assertions))]
    #[doc(hidden)]
    #[inline]
    fn find_map_inner<F, T>(&'ast self, mut predicate: F) -> Option<T>
    where
        F: FnMut(&'ast Node<'ast>) -> Option<T>,
    {
        self.fold_while(None, |acc, node| {
            debug_assert!(acc.is_none());

            match predicate(node) {
                Some(target) => ControlFlow::Break(Some(target)),
                None => ControlFlow::Continue(None),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, NodeKind, TextContent};

    #[test]
    fn find_map_not_item() {
        let text = "# Document header\nText";
        let document = Document::new(text);
        let ast = document.ast();

        let found = ast.find_map(|node| node.kind().is_strong().then(|| node));
        assert!(found.is_none());
    }

    #[test]
    fn find_map_with_duplicate() {
        let text = "# Header 1\n# Header 2";
        let document = Document::new(text);
        let ast = document.ast();

        let found = ast.find_map(|node| node.kind().is_heading().then(|| node));
        assert!(found.is_some());
    }

    #[test]
    fn find_map_heading() {
        let text = "# Header 1\n# Header 2";
        let document = Document::new(text);
        let ast = document.ast();

        let heading = ast
            .find_map(|node| {
                if let NodeKind::Heading(heading) = node.kind()
                    && heading
                        .as_plain_text()
                        .is_some_and(|text| text == "Header 2")
                {
                    return Some(heading);
                }

                None
            })
            .expect("not found");

        assert_eq!(heading.as_plain_text().unwrap(), "Header 2");
    }
}
