use crate::prelude::{Fold, FoldVisitorExt, Node, Visitor};
use std::ops::ControlFlow;

const PANIC_MESSAGE: &str = "check the initialization `FoldVisitor`";

struct FoldWhileVisitor<B, F> {
    predicate: F,
    data: Option<B>,
}

impl<B, F> FoldWhileVisitor<B, F> {
    pub const fn new(init: B, predicate: F) -> Self {
        Self {
            predicate,
            data: Some(init),
        }
    }
}

impl<'ast, B, F> Visitor<'ast> for FoldWhileVisitor<B, F>
where
    F: FnMut(B, &'ast Node<'ast>) -> ControlFlow<B, B>,
{
    fn pre_visit_node(&mut self, node: &'ast Node<'ast>) -> ControlFlow<()> {
        #[expect(clippy::expect_used)]
        let data = self.data.take().expect(PANIC_MESSAGE);
        let result = (self.predicate)(data, node);

        match result {
            ControlFlow::Continue(new_data) => {
                self.data = Some(new_data);

                ControlFlow::Continue(())
            }
            ControlFlow::Break(final_data) => {
                self.data = Some(final_data);

                ControlFlow::Break(())
            }
        }
    }
}

impl<B, F> Fold for FoldWhileVisitor<B, F> {
    type Output = B;

    fn finish(self) -> Self::Output {
        #[expect(clippy::expect_used)]
        self.data.expect(PANIC_MESSAGE)
    }
}

impl<'ast> Node<'ast> {
    pub fn fold_while<B, F>(&'ast self, init: B, predicate: F) -> B
    where
        F: FnMut(B, &'ast Node<'ast>) -> ControlFlow<B, B>,
    {
        let mut predicate = predicate;

        // Test optimization: use `dyn FnMut` in an inner function
        // to erase the closure type and prevent code bloat.
        self.fold_while_inner(init, &mut predicate)
    }

    #[cfg(debug_assertions)]
    #[doc(hidden)]
    #[inline(never)]
    fn fold_while_inner<B>(
        &'ast self,
        init: B,
        predicate: &mut dyn FnMut(B, &'ast Node<'ast>) -> ControlFlow<B, B>,
    ) -> B {
        let visitor = FoldWhileVisitor::new(init, predicate);
        self.fold_visitor(visitor)
    }

    #[cfg(not(debug_assertions))]
    #[doc(hidden)]
    #[inline]
    fn fold_while_inner<B, F>(&'ast self, init: B, predicate: F) -> B
    where
        F: FnMut(B, &'ast Node<'ast>) -> ControlFlow<B, B>,
    {
        let visitor = FoldWhileVisitor::new(init, predicate);
        self.fold_visitor(visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, HeadingLevel, Node, NodeKind, TextContent};
    use std::ops::ControlFlow;

    #[test]
    fn fold_while_not_sized() {
        let text = "# Hello\nWorld";
        let document = Document::new(text);
        let ast = document.ast();

        let predicate: &mut dyn FnMut(usize, &Node) -> ControlFlow<usize, usize> =
            &mut |acc, _| ControlFlow::Continue(acc + 1);

        let count = ast.fold_while(0usize, predicate);

        // Root + Heading + Text + Paragraph + Text
        assert_eq!(count, 5);
    }

    #[test]
    fn fold_while_counts_all_nodes() {
        let text = "# Hello\nWorld";
        let document = Document::new(text);
        let ast = document.ast();

        let count = ast.fold_while(0usize, |acc, _| ControlFlow::Continue(acc + 1));

        // Root + Heading + Text + Paragraph + Text
        assert_eq!(count, 5);
    }

    #[test]
    fn fold_while_on_empty_document() {
        let text = "";
        let document = Document::new(text);
        let ast = document.ast();

        let count = ast.fold_while(0usize, |acc, _| ControlFlow::Continue(acc + 1));

        // Only Root
        assert_eq!(count, 1);
    }

    #[test]
    fn fold_while_returns_initial_value() {
        let text = "# Hello";
        let document = Document::new(text);
        let ast = document.ast();

        let result = ast.fold_while(42usize, |acc, _| ControlFlow::Continue(acc));

        assert_eq!(result, 42);
    }

    #[test]
    fn fold_while_stops_on_break() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited_count = 0;
        let result = ast.fold_while(0usize, |acc, node| {
            visited_count += 1;

            if node.kind().is_heading() {
                ControlFlow::Break(acc + 99)
            } else {
                ControlFlow::Continue(acc + 1)
            }
        });

        assert_eq!(result, 100);
        assert!(visited_count < 5);
    }

    #[test]
    fn find_via_fold_while() {
        let text = "# First heading\n## Second heading";
        let document = Document::new(text);
        let ast = document.ast();

        let first_heading = ast
            .fold_while(None, |acc, node| {
                assert!(acc.is_none());

                if let Some(heading) = node.as_heading() {
                    ControlFlow::Break(Some(heading))
                } else {
                    ControlFlow::Continue(acc)
                }
            })
            .expect("not found");

        assert_eq!(first_heading.as_plain_text().unwrap(), "First heading");
    }

    #[test]
    fn any_via_fold_while() {
        let text = "# Hello\nWorld";
        let document = Document::new(text);
        let ast = document.ast();

        let has_heading = ast.fold_while(false, |_acc, node| {
            if node.kind().is_heading() {
                ControlFlow::Break(true)
            } else {
                ControlFlow::Continue(false)
            }
        });

        assert!(has_heading);
    }

    #[test]
    fn all_via_fold_while() {
        let text = "Just text, no headings";
        let document = Document::new(text);
        let ast = document.ast();

        let all_text = ast.fold_while(true, |acc, node| {
            if node.kind().is_heading() {
                ControlFlow::Break(false)
            } else {
                ControlFlow::Continue(acc)
            }
        });

        assert!(all_text);
    }

    #[test]
    fn all_returns_false_on_counterexample() {
        let text = "# Heading\nText";
        let document = Document::new(text);
        let ast = document.ast();

        let all_text = ast.fold_while(true, |acc, node| {
            if node.kind().is_heading() {
                ControlFlow::Break(false)
            } else {
                ControlFlow::Continue(acc)
            }
        });

        assert!(!all_text);
    }

    #[test]
    fn fold_while_collects_text() {
        let text = "Hello world";
        let document = Document::new(text);
        let ast = document.ast();

        let mut text = String::new();
        ast.fold_while((), |_, node| {
            if let NodeKind::Text(content) = node.kind() {
                text.push_str(content);
            }

            ControlFlow::Continue(())
        });

        assert_eq!(text, "Hello world");
    }

    #[test]
    fn fold_while_stops_collecting_at_heading() {
        let text = "Before heading\n\n# Heading\n\nAfter heading";
        let document = Document::new(text);
        let ast = document.ast();

        let text = ast.fold_while(String::new(), |mut acc, node| {
            if node.kind().is_heading() {
                return ControlFlow::Break(acc);
            }

            if let NodeKind::Text(content) = node.kind() {
                acc.push_str(content);
            }

            ControlFlow::Continue(acc)
        });

        assert!(text.contains("Before heading"));
        assert!(!text.contains("After heading"));
    }

    #[test]
    fn fold_while_visits_in_pre_order() {
        let text = "# Heading";
        let document = Document::new(text);
        let ast = document.ast();

        let mut order = Vec::new();
        let _ = ast.fold_while((), |_, node| {
            let name = match node.kind() {
                NodeKind::Root(_) => "Root",
                NodeKind::Heading(_) => "Heading",
                NodeKind::Text(_) => "Text",
                _ => "Other",
            };

            order.push(name);
            ControlFlow::Continue(())
        });

        // Root -> Heading -> Text
        assert_eq!(order, ["Root", "Heading", "Text"]);
    }

    #[test]
    fn fold_while_with_hashmap() {
        use std::collections::HashMap;

        let text = "# H1\n## H2\n### H3\n## H2 again";
        let document = Document::new(text);
        let ast = document.ast();

        let level_counts = ast.fold_while(HashMap::new(), |mut acc, node| {
            if let Some(heading) = node.as_heading() {
                *acc.entry(heading.level()).or_insert(0) += 1;
            }

            ControlFlow::Continue(acc)
        });

        assert_eq!(level_counts.len(), 3);
        assert_eq!(level_counts[&HeadingLevel::H1], 1);
        assert_eq!(level_counts[&HeadingLevel::H2], 2);
        assert_eq!(level_counts[&HeadingLevel::H3], 1);
    }

    #[test]
    fn fold_while_with_unit_accumulator() {
        let text = "# Hello";
        let document = Document::new(text);
        let ast = document.ast();

        let result = ast.fold_while((), |_, _| ControlFlow::Continue(()));

        assert_eq!(result, ());
    }

    #[test]
    fn fold_while_immediate_break() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited = 0;
        let result = ast.fold_while(0usize, |acc, _node| {
            visited += 1;
            ControlFlow::Break(acc + 42)
        });

        assert_eq!(result, 42);
        assert_eq!(visited, 1); // Only Root
    }

    #[test]
    fn fold_while_never_breaks() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited = 0;
        let _ = ast.fold_while((), |_, _node| {
            visited += 1;
            ControlFlow::Continue(())
        });

        assert!(visited > 0);
    }
}
