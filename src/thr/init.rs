#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

use crate::thr::{trap_handler, ThrTokens};
use drone_core::token::Token;

/// Threads initialization token.
///
/// # Safety
///
/// Must be defined only once for a particular set of threads.
pub unsafe trait ThrsInitToken: Token {
    /// The set of thread tokens.
    type ThrTokens: ThrTokens;

    /// Exception handler.
    const EXCEPTION_HANDLER: Option<unsafe extern "C" fn()>;

    /// Timer interrupt handler.
    const TIMER_INTERRUPT_HANDLER: Option<unsafe extern "C" fn()>;

    /// External interrupt handlers.
    const EXTERNAL_INTERRUPT_HANDLERS: &'static [Option<unsafe extern "C" fn()>];

    /// Sowtware interrupt handlers.
    const SOFTWARE_INTERRUPT_HANDLERS: &'static [unsafe extern "C" fn()];
}

/// Initializes the thread system and returns a set of thread tokens.
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn init<T: ThrsInitToken>(_token: T) -> T::ThrTokens {
    let mtvec = trap_handler::<T> as usize;
    unsafe {
        llvm_asm!("
            csrw mtvec, $0
        "   :
            : "r"(mtvec)
            :
            : "volatile");
        T::ThrTokens::take()
    }
}
