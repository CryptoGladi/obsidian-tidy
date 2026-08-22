use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlBlock;

super::impl_node_as!(HtmlBlock);
