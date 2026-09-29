#[repr(C)]
pub struct StageBattleMotherMagicant {
    _0: [u8; 0x830],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleMotherMagicant>(), 0x830);
        assert_eq!(offset_of!(StageBattleMotherMagicant, _0), 0x0);
    }
}
