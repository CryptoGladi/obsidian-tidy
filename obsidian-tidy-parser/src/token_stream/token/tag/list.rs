use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct List {
    pub number_item: Option<u64>,
}

impl List {
    #[must_use]
    pub const fn new(number_item: Option<u64>) -> Self {
        Self { number_item }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lol() {}
}
