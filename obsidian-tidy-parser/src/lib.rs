#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]
#![cfg_attr(feature = "no_std", no_std)]

extern crate alloc;

pub mod ast;
pub mod document;
pub mod markdown_lexer;
pub mod prelude;
pub mod token_stream;
pub mod visitor;

mod __private;
