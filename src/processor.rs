//! Common utility functions for working with RISC-V processors.

#![cfg_attr(feature = "std", allow(unreachable_code))]

use core::cell::UnsafeCell;

extern "C" {
    static STACK_POINTER: UnsafeCell<usize>;
}

/// Initializes the Stack Pointer.
///
/// # Safety
///
/// This function reverts the state of the Stack Pointer.
pub unsafe fn stack_pointer_init() {
    unsafe {
        asm!("mv sp, {}", in(reg) STACK_POINTER.get() as usize);
    }
}

/// Waits for interrupt.
///
/// This instruction provides a hint that the current HART can be stalled until
/// an interrupt might need servicing.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }
}
