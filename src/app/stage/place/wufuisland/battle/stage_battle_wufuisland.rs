#[repr(C)]
pub struct StageBattleWufuIsland {
    _0: [u8; 0x780],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleWufuIsland>(), 0x780);
        assert_eq!(offset_of!(StageBattleWufuIsland, _0), 0x0);
    }
}
