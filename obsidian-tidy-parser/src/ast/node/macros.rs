macro_rules! impl_node_as {
    ($name:ident) => {
        ::pastey::paste! {
            impl $crate::prelude::Node<'_> {
                #[must_use]
                pub const fn [<as_ $name:snake>](&self) -> Option<&$crate::prelude::Tag<'_, $name>> {
                    if let $crate::prelude::NodeKind::$name(data) = &self.kind {
                        return Some(data);
                    }

                    None
                }
            }
        }
    };
}

pub(crate) use impl_node_as;
