#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::panic))]
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::indexing_slicing))]

extern crate alloc;

pub(crate) mod __private;

pub(crate) mod markdown_lexer;
pub mod prelude;
pub mod token_stream;

pub use prelude::*;
