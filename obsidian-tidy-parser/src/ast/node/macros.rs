macro_rules! impl_node_as {
    ($name:ident, $for_return:path) => {
        ::pastey::paste! {
            impl $crate::prelude::Node<'_> {
                #[must_use]
                pub const fn [<as_ $name:snake>](&self) -> Option<&$crate::prelude::Tag<'_, $for_return>> {
                    if let $crate::prelude::NodeKind::$name(data) = &self.kind {
                        return Some(data);
                    }

                    None
                }
            }
        }
    };

    ($name:ident) => {
        $crate::ast::node::macros::impl_node_as!($name, $name);
    };
}

pub(crate) use impl_node_as;
