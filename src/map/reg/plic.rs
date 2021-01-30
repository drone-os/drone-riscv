use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Hart 0 M-Mode claim/complete.
    pub PLIC HART0_M_CLAIM_COMPLETE => {
        address => 0x0C20_0004;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Interrupt Claim/Complete for Hart 0 M-Mode.
            CLAIM_COMPLETE => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}
