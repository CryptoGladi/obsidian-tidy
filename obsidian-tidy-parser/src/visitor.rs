use crate::prelude::{CowStr, Heading, Node, NodeKind, Paragraph, Root, Tag};
use std::range::Range;

macro_rules! define_visitor {
    (
        tagged {
            $($variant:ident : $inner:ty),*
            $(,)?
        }
        leaf {
            $($data_variant:ident : $data_type:ty),*
            $(,)?
        }
        empty {
            $($empty_variant:ident),*
            $(,)?
        }
    ) => {
        ::pastey::paste! {
            #[allow(unused_variables)]
            pub trait Visitor<'a> {
                fn visit_node(&mut self, node: &Node<'a>) {
                    match &node.kind {
                        $(
                            NodeKind::$variant(tag) => self.[<visit_ $variant:snake>](tag, node.offset),
                        )*
                        $(
                            NodeKind::$data_variant(data) => self.[<visit_ $data_variant:snake>](data, node.offset),
                        )*
                        $(
                            NodeKind::$empty_variant => self.[<visit_ $empty_variant:snake>](node.offset),
                        )*
                    }
                }

                $(
                    fn [<pre_visit_ $variant:snake>](&mut self, tag: &Tag<'a, $inner>, offset: Range<usize>) {}

                    fn [<post_visit_ $variant:snake>](&mut self, tag: &Tag<'a, $inner>, offset: Range<usize>) {}

                    fn [<visit_ $variant:snake>](&mut self, tag: &Tag<'a, $inner>, offset: Range<usize>) {
                        self.[<pre_visit_ $variant:snake>](tag, offset);

                        for child in tag.children() {
                            self.visit_node(child);
                        }

                        self.[<post_visit_ $variant:snake>](tag, offset);
                    }
                )*

                $(
                    fn [<visit_ $data_variant:snake>](&mut self, data: &$data_type, offset: Range<usize>) {}
                )*

                $(
                    fn [<visit_ $empty_variant:snake>](&mut self, offset: Range<usize>) {}
                )*
            }
        }
    };
}

define_visitor! {
    tagged {
        Root: Root,
        Paragraph: Paragraph,
        Heading: Heading,
    }
    leaf {
        Text: CowStr<'a>,
    }
    empty {
        SoftBreak,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn fd() {
        let document = "# dsd fd ew";
        let ast = Parser::new(&document).ast();

        struct CountWord {
            count: usize,
        }

        impl Visitor<'_> for CountWord {
            fn visit_text(&mut self, text: &CowStr<'_>, _offset: Range<usize>) {
                self.count += text.split_whitespace().count();
            }
        }

        let mut count_word = CountWord { count: 0 };
        count_word.visit_node(&ast);

        assert_eq!(count_word.count, 3);
    }
}
