#[repr(C)]
pub struct StageBattlePickelWorld {
    _0: [u8; 0xA80],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattlePickelWorld>(), 0xA80);
        assert_eq!(offset_of!(StageBattlePickelWorld, _0), 0x0);
    }
}
