#![cfg_attr(feature = "std", allow(dead_code, unreachable_code, unused_imports))]

use crate::thr::SoftThread;
use core::ptr::{read_volatile, write_volatile};
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

    /// Base address of TIMER Memory Map.
    const TIMER_BASE: usize;

    /// Base address of PLIC Memory Map.
    const PLIC_BASE: usize;
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

/// Machine trap handler function.
///
/// # Safety
///
/// The function is not reentrant.
#[naked]
pub unsafe extern "C" fn trap_handler<T: ThrsInitToken>() {
    unsafe {
        asm!(
            ".align 4", // TODO https://github.com/rust-lang/rust/issues/82232
            "    addi sp, sp, -80",
            "    sw a0, 76(sp)",
            "    sw a1, 72(sp)",
            "    csrr a0, mcause",
            "    li a1, 11",
            "    bne a0, a1, 0f",
            "    addi sp, sp, 80",
            "    lw a0, 12(sp)",
            "    csrw mepc, a0",
            "    j 1f",
            "0:  sw a2, 68(sp)",
            "    sw a3, 64(sp)",
            "    sw a4, 60(sp)",
            "    sw a5, 56(sp)",
            "    sw a6, 52(sp)",
            "    sw a7, 48(sp)",
            "    sw t0, 44(sp)",
            "    sw t1, 40(sp)",
            "    sw t2, 36(sp)",
            "    sw t3, 32(sp)",
            "    sw t4, 28(sp)",
            "    sw t5, 24(sp)",
            "    sw t6, 20(sp)",
            "    sw ra, 16(sp)",
            "    jal {pending_idx}",
            "    beqz a0, 1f",
            "    addi a0, a0, -1",
            "    jal {will_preempt}",
            "    beqz a0, 1f",
            "    csrr a0, mepc",
            "    sw a0, 12(sp)",
            "    la a0, {preempt}",
            "    csrw mepc, a0",
            "    mret",
            "1:  lw ra, 16(sp)",
            "    lw t6, 20(sp)",
            "    lw t5, 24(sp)",
            "    lw t4, 28(sp)",
            "    lw t3, 32(sp)",
            "    lw t2, 36(sp)",
            "    lw t1, 40(sp)",
            "    lw t0, 44(sp)",
            "    lw a7, 48(sp)",
            "    lw a6, 52(sp)",
            "    lw a5, 56(sp)",
            "    lw a4, 60(sp)",
            "    lw a3, 64(sp)",
            "    lw a2, 68(sp)",
            "    lw a1, 72(sp)",
            "    lw a0, 76(sp)",
            "    addi sp, sp, 80",
            "    mret",
            pending_idx = sym pending_idx::<T>,
            will_preempt = sym will_preempt::<T::Thread>,
            preempt = sym preempt::<T>,
            options(noreturn),
        );
    }
}

#[naked]
unsafe extern "C" fn preempt<T: ThrsInitToken>() {
    unsafe {
        asm!(
            "jal {soft_preempt}",
            "ecall",
            soft_preempt = sym soft_preempt::<T::Thread>,
            options(noreturn),
        );
    }
}

unsafe extern "C" fn pending_idx<T: ThrsInitToken>(mcause: usize) -> u16 {
    if mcause & 1 << 31 == 0 {
        return T::EXCEPTION_HANDLER;
    } else if mcause == 1 << 31 | 7 {
        unsafe {
            write_volatile((T::TIMER_BASE + 0x8) as *mut u32, 0xFFFF_FFFF);
            write_volatile((T::TIMER_BASE + 0xC) as *mut u32, 0xFFFF_FFFF);
        }
        return T::TIMER_HANDLER;
    } else if mcause == 1 << 31 | 3 {
        // Software Interrupt. Unimplemented.
    } else if mcause == 1 << 31 | 11 {
        let num = unsafe { read_volatile((T::PLIC_BASE + 0x0020_0004) as *const u32) };
        if num > 0 {
            if let Some(&idx) = T::EXTERNAL_HANDLERS.get(num as usize - 1) {
                return idx;
            }
        }
    }
    0
}

unsafe extern "C" fn will_preempt<T: SoftThread>(thr_idx: u16) -> bool {
    unsafe { T::will_preempt(thr_idx) }
}

extern "C" fn soft_preempt<T: SoftThread>() {
    T::preempt();
}
