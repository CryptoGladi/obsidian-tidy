use super::{Item, Tag};
use crate::token_stream::token::{List as TokenList, ListDelimiter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub start_number: Option<u64>,
    pub delimiter: Option<ListDelimiter>,
}

impl From<TokenList> for List {
    fn from(value: TokenList) -> Self {
        // TODO использовать это везде, чтобы точно не забыть указать новые поля
        // При обновлении версий
        let TokenList {
            start_number,
            delimiter,
        } = value;

        Self {
            start_number,
            delimiter,
        }
    }
}

impl<'ast> Tag<'ast, List> {
    #[must_use]
    pub const fn start_number(&self) -> Option<u64> {
        self.kind.start_number
    }

    #[must_use]
    pub const fn delimiter(&self) -> Option<ListDelimiter> {
        self.kind.delimiter
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
