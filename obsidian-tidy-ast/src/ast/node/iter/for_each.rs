use crate::prelude::Node;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn for_each<F>(&'ast self, predicate: F)
    where
        F: FnMut(&'ast Node<'ast>),
    {
        let mut predicate = predicate;

        self.fold((), |(), node| predicate(node));
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, HeadingLevel, NodeKind};

    #[test]
    fn for_each_while_visits_in_pre_order() {
        let text = "# Heading";
        let document = Document::new(text);
        let ast = document.ast();

        let mut order = Vec::new();
        ast.for_each(|node| {
            let name = match node.kind() {
                NodeKind::Root(_) => "Root",
                NodeKind::Heading(_) => "Heading",
                NodeKind::Text(_) => "Text",
                _ => "Other",
            };

            order.push(name);
        });

        // Root -> Heading -> Text
        assert_eq!(order, ["Root", "Heading", "Text"]);
    }

    #[test]
    fn for_each_while_with_hashmap() {
        use std::collections::HashMap;

        let text = "# H1\n## H2\n### H3\n## H2 again";
        let document = Document::new(text);
        let ast = document.ast();

        let mut level_counts = HashMap::new();
        ast.for_each(|node| {
            if let Some(heading) = node.as_heading() {
                *level_counts.entry(heading.level()).or_insert(0) += 1;
            }
        });

        assert_eq!(level_counts.len(), 3);
        assert_eq!(level_counts[&HeadingLevel::H1], 1);
        assert_eq!(level_counts[&HeadingLevel::H2], 2);
        assert_eq!(level_counts[&HeadingLevel::H3], 1);
    }
}
