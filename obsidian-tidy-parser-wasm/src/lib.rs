#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod ast;
pub mod prelude;
pub mod token_stream;
