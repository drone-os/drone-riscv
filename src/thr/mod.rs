//! The Threads module.
//!
//! **NOTE** This module documentation should be viewed as a continuation of
//! [the `drone_core` documentation](drone_core::thr).

pub mod prelude;

mod init;

#[doc(no_inline)]
pub use drone_core::thr::*;

pub use self::init::{init, ThrsInitToken};

use crate::{map::reg::plic, processor::csr_read_mcause, reg::prelude::*};
use drone_core::token::Token;

/// Defines a thread pool driven by CLINT (Core Local Interrupter).
///
/// See [the module level documentation](self) for details.
#[doc(inline)]
pub use drone_riscv_macros::thr_clint as clint;

/// Machine trap handler function.
///
/// # Safety
///
/// The function is not reentrant.
pub unsafe extern "C" fn trap_handler<T: ThrsInitToken>() {
    let mut thr_idx = 0;
    let mcause = csr_read_mcause();
    if mcause & 1 << 31 == 0 {
        thr_idx = T::EXCEPTION_HANDLER;
    } else if mcause == 1 << 31 | 7 {
        thr_idx = T::TIMER_HANDLER;
    } else if mcause == 1 << 31 | 3 {
        // software interrupt
    } else if mcause == 1 << 31 | 11 {
        let plic_claim_complete = unsafe { plic::Hart0MClaimComplete::<Srt>::take() };
        let num = plic_claim_complete.load().claim_complete();
        if num > 0 {
            if let Some(&idx) = T::EXTERNAL_HANDLERS.get(num as usize - 1) {
                thr_idx = idx;
            }
            plic_claim_complete.store(|r| r.write_claim_complete(num));
        }
    }
    if thr_idx > 0 && unsafe { T::Thread::will_preempt(thr_idx - 1) } {
        T::Thread::preempt(); // TODO set stack
    }
}
