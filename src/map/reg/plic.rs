use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Hart 0 M-Mode claim/complete.
    pub mod PLIC HART0_M_CLAIM_COMPLETE;
    0x0C20_0004 0x20 0x0000_0000
    RReg WReg;
    /// Interrupt Claim/Complete for Hart 0 M-Mode.
    CLAIM_COMPLETE { 0 32 RRRegField WWRegField }
}
