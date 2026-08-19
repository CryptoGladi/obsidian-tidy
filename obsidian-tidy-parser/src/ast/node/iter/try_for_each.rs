use crate::prelude::Node;
use core::ops::ControlFlow;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn try_for_each<F, E>(&'ast self, mut predicate: F) -> Result<(), E>
    where
        F: FnMut(&'ast Node<'ast>) -> ControlFlow<E>,
    {
        self.fold_while(Ok(()), |_, node| match predicate(node) {
            ControlFlow::Break(err) => ControlFlow::Break(Err(err)),
            ControlFlow::Continue(()) => ControlFlow::Continue(Ok(())),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, HeadingLevel, NodeKind};
    use core::ops::ControlFlow;

    #[test]
    fn try_for_each_while_visits_in_pre_order() {
        let text = "# Heading **super**";
        let document = Document::new(text);
        let ast = document.ast();

        let mut order = Vec::new();
        let break_node = ast.try_for_each(|node| {
            let name = match node.kind() {
                NodeKind::Root(_) => "Root",
                NodeKind::Heading(_) => "Heading",
                NodeKind::Text(_) => "Text",
                _ => return ControlFlow::Break(node),
            };

            order.push(name);
            ControlFlow::Continue(())
        });

        assert!(break_node.is_err());
    }

    #[test]
    fn try_for_each_while_with_hashmap() {
        use std::collections::HashMap;

        let text = "# H1\n## H2\n### H3\n## H2 again";
        let document = Document::new(text);
        let ast = document.ast();

        let mut level_counts = HashMap::new();
        ast.try_for_each::<_, ()>(|node| {
            if let Some(heading) = node.as_heading() {
                *level_counts.entry(heading.level()).or_insert(0) += 1;
            }

            ControlFlow::Continue(())
        })
        .unwrap();

        assert_eq!(level_counts.len(), 3);
        assert_eq!(level_counts[&HeadingLevel::H1], 1);
        assert_eq!(level_counts[&HeadingLevel::H2], 2);
        assert_eq!(level_counts[&HeadingLevel::H3], 1);
    }

    #[test]
    fn try_for_each_while_with_unit_accumulator() {
        let text = "# Hello";
        let document = Document::new(text);
        let ast = document.ast();

        let result = ast.try_for_each::<_, ()>(|_| ControlFlow::Continue(()));
        assert!(result.is_ok());
    }

    #[test]
    fn try_for_each_while_immediate_break() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited = 0;
        let result = ast.try_for_each(|_| {
            visited += 1;
            ControlFlow::Break(42)
        });

        assert_eq!(result, Err(42));
        assert_eq!(visited, 1); // Only Root
    }

    #[test]
    fn try_for_each_while_never_breaks() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited = 0;
        let _ = ast.try_for_each::<_, ()>(|_| {
            visited += 1;
            ControlFlow::Continue(())
        });

        assert!(visited > 0);
    }
}
