#[repr(C)]
pub struct StageBattleWarioGamer {
    _0: [u8; 0xF50],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleWarioGamer>(), 0xF50);
        assert_eq!(offset_of!(StageBattleWarioGamer, _0), 0x0);
    }
}
