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

/// A set of thread tokens.
///
/// # Safety
///
/// Must contain only thread tokens.
pub unsafe trait ThrTokens: Token {}

/// Machine trap handler function.
///
/// # Safety
///
/// The function is not reentrant.
pub unsafe extern "C" fn trap_handler<T: ThrsInitToken>() {
    let mcause = csr_read_mcause();
    if mcause & 1 << 31 == 0 {
        if let Some(handler) = T::EXCEPTION_HANDLER {
            unsafe { handler() };
        }
    } else if mcause == 1 << 31 | 7 {
        if let Some(handler) = T::TIMER_INTERRUPT_HANDLER {
            unsafe { handler() };
        }
    } else if mcause == 1 << 31 | 3 {
        // TODO Software interrupt
    } else if mcause == 1 << 31 | 11 {
        let plic_claim_complete = unsafe { plic::Hart0MClaimComplete::<Srt>::take() };
        let source = plic_claim_complete.load().claim_complete();
        if source > 0 {
            if let Some(Some(handler)) = T::EXTERNAL_INTERRUPT_HANDLERS.get(source as usize - 1) {
                unsafe { handler() };
            }
            plic_claim_complete.store(|r| r.write_claim_complete(source));
        }
    }
}
