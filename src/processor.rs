//! Common utility functions for working with RISC-V processors.

#![cfg_attr(feature = "std", allow(unreachable_code, unused_mut))]

/// Waits for interrupt.
///
/// This instruction provides a hint that the current HART can be stalled until
/// an interrupt might need servicing.
#[inline]
pub fn wait_for_int() {
    #[cfg(feature = "std")]
    return unimplemented!();
    unsafe { llvm_asm!("wfi" :::: "volatile") };
}

/// Read MCAUSE CSR (Control and Status Register).
#[inline]
#[must_use]
pub fn csr_read_mcause() -> usize {
    let mcause;
    unsafe {
        llvm_asm!("
            csrr $0, mcause
        "   : "=r"(mcause)
            :
            :
            : "volatile");
    }
    mcause
}
