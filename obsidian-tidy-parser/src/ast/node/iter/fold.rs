use crate::prelude::Node;
use std::ops::ControlFlow;

impl<'ast> Node<'ast> {
    #[inline]
    pub fn fold<B, F>(&'ast self, init: B, predicate: F) -> B
    where
        F: FnMut(B, &'ast Node<'ast>) -> B,
    {
        let mut predicate = predicate;

        self.fold_while(init, |acc, node| {
            let acc = predicate(acc, node);
            ControlFlow::Continue(acc)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Document, HeadingLevel, NodeKind, TextContent};

    #[test]
    fn fold_counts_all_nodes() {
        let text = "# Hello\nWorld";
        let document = Document::new(text);
        let ast = document.ast();

        let count = ast.fold(0usize, |acc, _| acc + 1);

        // Root + Heading + Text + Paragraph + Text = 5
        assert_eq!(count, 5);
    }

    #[test]
    fn fold_on_empty_document() {
        let text = "";
        let document = Document::new(text);
        let ast = document.ast();

        let count = ast.fold(0usize, |acc, _| acc + 1);

        // Only Root
        assert_eq!(count, 1);
    }

    #[test]
    fn fold_returns_initial_value_for_no_match() {
        let text = "# Hello";
        let document = Document::new(text);
        let ast = document.ast();

        let result = ast.fold(
            42usize,
            |acc, node| {
                if node.kind.is_strong() { acc + 1 } else { acc }
            },
        );

        assert_eq!(result, 42);
    }

    #[test]
    fn fold_counts_headings() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let heading_count = ast.fold(0usize, |acc, node| {
            if node.kind().is_heading() {
                acc + 1
            } else {
                acc
            }
        });

        assert_eq!(heading_count, 3);
    }

    // === Аккумуляция в строку ===

    #[test]
    fn fold_collects_text_content() {
        let text = "Hello world";
        let document = Document::new(text);
        let ast = document.ast();

        let text = ast.fold(String::new(), |mut acc, node| {
            if let NodeKind::Text(content) = node.kind() {
                acc.push_str(content);
            }

            acc
        });

        assert_eq!(text, "Hello world");
    }

    #[test]
    fn fold_collects_heading_texts() {
        let text = "# First\n## Second\n### Third";
        let document = Document::new(text);
        let ast = document.ast();

        let headings_text = ast.fold(String::new(), |mut acc, node| {
            if let Some(heading) = node.as_heading() {
                if !acc.is_empty() {
                    acc.push_str(", ");
                }

                if let Some(text) = heading.as_plain_text() {
                    acc.push_str(text);
                }
            }

            acc
        });

        assert_eq!(headings_text, "First, Second, Third");
    }

    #[test]
    fn fold_collects_all_text_nodes() {
        let text = "Hello **bold** world";
        let document = Document::new(text);
        let ast = document.ast();

        let texts = ast.fold(Vec::new(), |mut acc, node| {
            if let NodeKind::Text(content) = node.kind() {
                acc.push(content.trim().to_string());
            }

            acc
        });

        assert_eq!(texts, ["Hello", "bold", "world"]);
    }

    #[test]
    fn fold_visits_nodes_in_pre_order() {
        let text = "# Heading";
        let document = Document::new(text);
        let ast = document.ast();

        let order = ast.fold(Vec::new(), |mut order, node| {
            let name = match node.kind() {
                NodeKind::Root(_) => "Root",
                NodeKind::Heading(_) => "Heading",
                NodeKind::Text(_) => "Text",
                _ => "Other",
            };

            order.push(name);
            order
        });

        // Root -> Heading -> Text
        assert_eq!(order, ["Root", "Heading", "Text"]);
    }

    #[test]
    fn fold_with_hashmap_accumulator() {
        use std::collections::HashMap;

        let text = "# H1\n## H2\n### H3\n## H2 again";
        let document = Document::new(text);
        let ast = document.ast();

        let level_counts = ast.fold(HashMap::new(), |mut acc, node| {
            if let Some(heading) = node.as_heading() {
                *acc.entry(heading.level()).or_insert(0) += 1;
            }

            acc
        });

        assert_eq!(level_counts.len(), 3);
        assert_eq!(level_counts[&HeadingLevel::H1], 1);
        assert_eq!(level_counts[&HeadingLevel::H2], 2);
        assert_eq!(level_counts[&HeadingLevel::H3], 1);
    }

    #[test]
    // Не оптимальный код! Требуется остановка!
    fn fold_with_option_accumulator() {
        let text = "# First heading\n## Second heading";
        let document = Document::new(text);
        let ast = document.ast();

        let first_heading = ast.fold(None, |acc, node| {
            if acc.is_none() {
                if let Some(heading) = node.as_heading() {
                    return Some(heading.level());
                }
            }

            acc
        });

        assert_eq!(first_heading, Some(HeadingLevel::H1));
    }

    #[test]
    fn fold_with_unit_accumulator() {
        let text = "# Hello";
        let document = Document::new(text);
        let ast = document.ast();

        let result = ast.fold((), |_, _| {});

        assert_eq!(result, ());
    }

    #[test]
    fn fold_short_circuits_with_any_semantics() {
        let text = "# H1\n## H2\n### H3";
        let document = Document::new(text);
        let ast = document.ast();

        let mut visited_count = 0;
        let found = ast.fold(None, |acc, node| {
            visited_count += 1;

            if acc.is_none() && node.kind().is_heading() {
                return Some(node);
            }

            acc
        });

        assert!(found.is_some());
        assert!(visited_count >= 2);
    }
}
