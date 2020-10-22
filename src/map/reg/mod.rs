//! Core RISC-V register mappings.

#[path = "."]
mod inner {
    mod plic;

    pub use self::plic::*;
}

use drone_core::reg;

reg::tokens! {
    #[doc(hidden)]
    pub macro riscv_reg_tokens_inner;
    super::inner;
    crate::map::reg;

    /// Platform-Level Interrupt Controller.
    pub mod PLIC {
        HART0_M_CLAIM_COMPLETE;
    }
}

// Workaround the `macro_expanded_macro_exports_accessed_by_absolute_paths`
// error.
#[doc(hidden)]
#[macro_export]
macro_rules! riscv_reg_tokens {
    ($($tt:tt)*) => {
        use $crate::riscv_reg_tokens_inner;
        riscv_reg_tokens_inner!($($tt)*);
    };
}
