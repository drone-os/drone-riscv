//! Procedural macros for [drone-riscv].
//!
//! [drone-riscv]: https://github.com/drone-os/drone-riscv

#![feature(unsafe_block_in_unsafe_fn)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(clippy::pedantic)]

extern crate proc_macro;

mod thr;

use proc_macro::TokenStream;

#[proc_macro]
pub fn thr(input: TokenStream) -> TokenStream {
    thr::proc_macro(input)
}
