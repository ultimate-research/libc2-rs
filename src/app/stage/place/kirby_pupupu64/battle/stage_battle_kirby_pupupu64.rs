#[repr(C)]
pub struct StageBattleKirbyPupupu64 {
    _0: [u8; 0x8D0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleKirbyPupupu64>(), 0x8D0);
        assert_eq!(offset_of!(StageBattleKirbyPupupu64, _0), 0x0);
    }
}
