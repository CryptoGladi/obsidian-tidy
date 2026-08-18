use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct List {
    pub number_item: Option<u64>,
}

impl List {
    pub fn new(number_item: Option<u64>) -> Self {
        Self { number_item }
    }
}
