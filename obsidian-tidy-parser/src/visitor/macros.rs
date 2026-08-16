/// Generates a `Visitor` trait for traversing an AST.
///
/// This macro creates a trait with methods for visiting each node type in the AST.
/// It supports three categories of nodes:
///
/// - **tagged**: Nodes with children (e.g., `Root`, `Paragraph`, `Heading`).
/// - **leaf**: Nodes with data but no children (e.g., `Text`).
/// - **empty**: Nodes without data or children (e.g., `SoftBreak`).
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
            /// A trait for visiting AST nodes.
            ///
            /// Implement this trait to traverse and process AST nodes.
            /// All methods have default implementations that do nothing,
            /// so you only need to override the methods you care about.
            #[allow(unused_variables)]
            #[allow(clippy::ptr_arg)]
            pub trait Visitor<'ast> {
                /// Called before [`Self::visit_node`]
                ///
                /// Return `ControlFlow::Break(())` to stop traversal.
                fn pre_visit_node(&mut self, node: &'ast $crate::prelude::Node<'ast>) -> ::std::ops::ControlFlow<()> {
                    ::std::ops::ControlFlow::Continue(())
                }

                /// Called after [`Self::visit_node`]
                fn post_visit_node(&mut self, node: &'ast $crate::prelude::Node<'ast>) {}

                /// Visits a node by dispatching to the appropriate method based on node kind.
                ///
                /// This is the main entry point for traversal. It matches on `node.kind`
                /// and calls the corresponding `visit_*` method.
                ///
                /// Return `ControlFlow::Break(())` to stop traversal.
                fn visit_node(&mut self, node: &'ast $crate::prelude::Node<'ast>) -> ::std::ops::ControlFlow<()> {
                    self.pre_visit_node(node)?;

                    match &node.kind() {
                        $(
                            $crate::prelude::NodeKind::$variant(tag) =>
                                self.[<visit_ $variant:snake>](tag, node.offset())?,
                        )*
                        $(
                            $crate::prelude::NodeKind::$data_variant(data) =>
                                self.[<visit_ $data_variant:snake>](data, node.offset())?,
                        )*
                        $(
                            $crate::prelude::NodeKind::$empty_variant =>
                                self.[<visit_ $empty_variant:snake>](node.offset())?,
                        )*
                    }

                    self.post_visit_node(node);
                    ::std::ops::ControlFlow::Continue(())
                }

                $(
                    #[doc = concat!(
                        "Called before visiting children of `",
                        stringify!($variant),
                        "` node."
                    )]
                    #[doc = concat!(
                        "See also [`",
                        stringify!([<visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<visit_ $variant:snake>]),
                        ") and [`",
                        stringify!([<post_visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<post_visit_ $variant:snake>]),
                        ")."
                    )]
                    #[doc = "Return `ControlFlow::Break(())` to stop traversal."]
                    fn [<pre_visit_ $variant:snake>](&mut self, tag: &'ast $crate::prelude::Tag<'ast, $inner>,
                        offset: ::std::range::Range<usize>) -> ::std::ops::ControlFlow<()> {
                        ::std::ops::ControlFlow::Continue(())
                    }

                    #[doc = concat!(
                        "Called after visiting children of `",
                        stringify!($variant),
                        "` node."
                    )]
                    #[doc = concat!(
                        "See also [`",
                        stringify!([<pre_visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<pre_visit_ $variant:snake>]),
                        ") and [`",
                        stringify!([<visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<visit_ $variant:snake>]),
                        ")."
                    )]
                    fn [<post_visit_ $variant:snake>](&mut self, tag: &'ast $crate::prelude::Tag<'ast, $inner>,
                        offset: ::std::range::Range<usize>) { }

                    #[doc = concat!(
                        "Visits `",
                        stringify!($variant),
                        "` node and its children."
                    )]
                    #[doc = concat!(
                        "Calls [`",
                        stringify!([<pre_visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<pre_visit_ $variant:snake>]),
                        ") before visiting children and [`",
                        stringify!([<post_visit_ $variant:snake>]),
                        "`](Self::",
                        stringify!([<post_visit_ $variant:snake>]),
                        ") after."
                    )]
                    #[doc = "If you override this method, you are"]
                    #[doc = "responsible for calling these hooks yourself if needed"]
                    #[doc = "Return `ControlFlow::Break(())` to stop traversal."]
                    fn [<visit_ $variant:snake>](&mut self, tag: &'ast $crate::prelude::Tag<'ast, $inner>,
                        offset: ::std::range::Range<usize>) -> ::std::ops::ControlFlow<()> {
                        self.[<pre_visit_ $variant:snake>](tag, offset)?;

                        for child in tag.children() {
                            self.visit_node(child)?;
                        }

                        self.[<post_visit_ $variant:snake>](tag, offset);
                        ::std::ops::ControlFlow::Continue(())
                    }
                )*

                $(
                    #[doc = concat!(
                        "Visits `",
                        stringify!($data_variant),
                        "` leaf node with data."
                    )]
                    #[doc = concat!(
                        "Leaf nodes have data but no children. The default implementation does nothing."
                    )]
                    #[doc = "Return `ControlFlow::Break(())` to stop traversal."]
                    fn [<visit_ $data_variant:snake>](&mut self, data: &'ast $data_type,
                        offset: ::std::range::Range<usize>) -> ::std::ops::ControlFlow<()> {
                        ::std::ops::ControlFlow::Continue(())
                    }
                )*

                $(
                    #[doc = concat!(
                        "Visits `",
                        stringify!($empty_variant),
                        "` empty node."
                    )]
                    #[doc = concat!(
                        "Empty nodes have no data and no children. The default implementation does nothing."
                    )]
                    #[doc = "Return `ControlFlow::Break(())` to stop traversal."]
                    fn [<visit_ $empty_variant:snake>](&mut self,
                        offset: ::std::range::Range<usize>) -> ::std::ops::ControlFlow<()> {
                        ::std::ops::ControlFlow::Continue(())
                    }
                )*
            }
        }
    };
}

pub(crate) use define_visitor;
