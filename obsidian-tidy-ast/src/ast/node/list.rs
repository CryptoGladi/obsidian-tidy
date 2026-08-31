use super::{Item, Tag};
pub use obsidian_tidy_lexer::{List, ListDelimiter};

impl<'ast> Tag<'ast, List> {
    #[must_use]
    pub const fn start_number(&self) -> Option<u64> {
        self.kind.start_number()
    }

    #[must_use]
    pub const fn delimiter(&self) -> Option<ListDelimiter> {
        self.kind.delimiter_opt()
    }

    pub fn items(&'ast self) -> impl Iterator<Item = &'ast Tag<'ast, Item>> {
        self.children().iter().filter_map(|node| node.as_item())
    }

    #[must_use]
    pub fn count_items(&self) -> usize {
        self.items().count()
    }
}

super::impl_node_as!(List);
