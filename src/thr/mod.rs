//! The Threads module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::thr).

pub mod prelude;

mod init;

#[doc(no_inline)]
pub use drone_core::thr::*;

pub use self::init::{init, ThrsInitToken};

/// Defines a thread pool driven by CLINT (Core Local Interrupter).
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_riscv_macros::thr_clint as clint;
