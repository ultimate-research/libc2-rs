#[repr(C)]
pub struct StageBattleKartCircuitX {
    _0: [u8; 0x920],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleKartCircuitX>(), 0x920);
        assert_eq!(offset_of!(StageBattleKartCircuitX, _0), 0x0);
    }
}
