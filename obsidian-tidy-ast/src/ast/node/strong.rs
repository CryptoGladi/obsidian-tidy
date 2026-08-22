use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Strong;

impl Strong {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

super::impl_node_as!(Strong);
