//! Common utility functions for working with RISC-V processors.

#![cfg_attr(feature = "std", allow(unreachable_code))]

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

/// Read MCAUSE CSR (Control and Status Register).
#[inline]
#[must_use]
pub fn csr_read_mcause() -> usize {
    #[cfg(feature = "std")]
    return unimplemented!();
    #[cfg(not(feature = "std"))]
    unsafe {
        let mcause;
        asm!(
            "csrr {0}, mcause",
            out(reg) mcause,
            options(nomem, nostack, preserves_flags),
        );
        mcause
    }
}
