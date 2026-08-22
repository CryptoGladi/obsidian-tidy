#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub(crate) mod __private;

pub mod markdown_lexer;
pub mod prelude;
pub mod token_stream;

pub use prelude::*;
