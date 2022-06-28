// Does not work with backticks.
// #![doc = include_str!("../README.md")]

//! See [README.md](https://github.com/thomasbratt/clockwork/blob/main/core/README.md) for a description
//!
mod bytecode;
mod execution;
mod instruction;

pub use bytecode::*;
pub use execution::*;
pub use instruction::*;
