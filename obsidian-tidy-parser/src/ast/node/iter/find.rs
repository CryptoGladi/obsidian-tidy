use crate::prelude::Node;
use std::ops::ControlFlow;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn find<F>(&'ast self, predicate: F) -> Option<&'ast Node<'ast>>
    where
        F: FnMut(&'ast Node<'ast>) -> bool,
    {
        let mut predicate = predicate;

        self.fold_while(None, |acc, node| {
            debug_assert!(acc.is_none());

            if predicate(node) {
                ControlFlow::Break(Some(node))
            } else {
                ControlFlow::Continue(None)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, NodeKind, TextContent};

    #[test]
    fn find_not_item() {
        let text = "# Document header\nText";
        let document = Document::new(text);
        let ast = document.ast();

        let found = ast.find(|node| node.kind().is_strong());
        assert!(found.is_none());
    }

    #[test]
    fn find_with_duplicate() {
        let text = "# Header 1\n# Header 2";
        let document = Document::new(text);
        let ast = document.ast();

        let found = ast.find(|node| node.kind().is_heading());
        assert!(found.is_some());
    }

    #[test]
    fn find_heading() {
        let text = "# Header 1\n# Header 2";
        let document = Document::new(text);
        let ast = document.ast();

        let heading = ast
            .find(|node| {
                if let NodeKind::Heading(heading) = node.kind() {
                    return heading
                        .as_plain_text()
                        .is_some_and(|text| text == "Header 2");
                }

                false
            })
            .expect("not found");

        assert_eq!(
            heading.as_heading().unwrap().as_plain_text().unwrap(),
            "Header 2"
        );
    }
}
