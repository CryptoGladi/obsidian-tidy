use alloc::borrow::Cow;
pub use obsidian_tidy_lexer::CodeBlock;
use serde::{Deserialize, Serialize};

super::impl_node_as!(CodeBlock, CodeBlock<'_>);
