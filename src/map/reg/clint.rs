use crate::reg::prelude::*;
use drone_core::reg;

reg! {
    /// Hart 0 M-Mode software interrupt pending.
    pub CLINT HART0_MSIP => {
        address => 0x0200_0000;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Software interrupt pending for Hart 0 M-Mode.
            MSIP => { offset => 0; width => 1; traits => { RRRegField WWRegField } };
        };
    };

    /// Hart 0 M-Mode timer compare (low 32 bits).
    pub CLINT HART0_MTIMECMP_LOW => {
        address => 0x0200_4000;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer compare for Hart 0 M-Mode.
            MTIMECMP => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// Hart 0 M-Mode timer compare (high 32 bits).
    pub CLINT HART0_MTIMECMP_HIGH => {
        address => 0x0200_4004;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer compare for Hart 0 M-Mode.
            MTIMECMP => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// Hart 0 M-Mode timer (low 32 bits).
    pub CLINT HART0_MTIME_LOW => {
        address => 0x0200_BFF8;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer for Hart 0 M-Mode.
            MTIME => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// Hart 0 M-Mode timer (high 32 bits).
    pub CLINT HART0_MTIME_HIGH => {
        address => 0x0200_BFFC;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer for Hart 0 M-Mode.
            MTIME => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}
