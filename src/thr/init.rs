#![cfg_attr(feature = "std", allow(unreachable_code, unused_imports))]

use crate::thr::{trap_handler, SoftThread};
use drone_core::token::Token;

/// Threads initialization token.
///
/// # Safety
///
/// * Must be defined only once for a particular set of threads.
/// * `ThrTokens` type must contain only thread tokens.
pub unsafe trait ThrsInitToken: Token {
    /// The thread type.
    type Thread: SoftThread;

    /// The set of thread tokens.
    type ThrTokens: Token;

    /// Exception handler.
    const EXCEPTION_HANDLER: u16;

    /// Timer interrupt handler.
    const TIMER_HANDLER: u16;

    /// External interrupt handlers.
    const EXTERNAL_HANDLERS: &'static [u16];
}

/// Initializes the thread system and returns a set of thread tokens.
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init<T: ThrsInitToken>(_token: T) -> T::ThrTokens {
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        let mtvec = trap_handler::<T> as usize;
        asm!(
            "csrw mtvec, {0}",
            in(reg) mtvec,
            options(nomem, nostack, preserves_flags),
        );
        T::ThrTokens::take()
    }
}
