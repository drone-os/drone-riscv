//! The Memory-Mapped Registers prelude.
//!
//! The purpose of this module is to alleviate imports of many common `reg`
//! traits by adding a glob import to the top of `reg` heavy modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_riscv::reg::prelude::*;
//! ```

#[doc(no_inline)]
pub use drone_core::reg::prelude::*;
