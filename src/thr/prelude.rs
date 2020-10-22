//! The Threads prelude.
//!
//! The purpose of this module is to alleviate imports of many common thread
//! token traits by adding a glob import to the top of thread token heavy
//! modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! use drone_riscv::thr::prelude::*;
//! ```

#[doc(no_inline)]
pub use drone_core::thr::prelude::*;
