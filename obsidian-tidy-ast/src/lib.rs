#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]
#![cfg_attr(all(not(test), feature = "no_std"), no_std)]

extern crate alloc;

pub mod ast;
pub mod document;
pub mod prelude;
pub mod visitor;

mod __private;

pub use prelude::*;
