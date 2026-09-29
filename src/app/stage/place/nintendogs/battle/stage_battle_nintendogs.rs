#[repr(C)]
pub struct StageBattleNintendogs {
    _0: [u8; 0x780],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleNintendogs>(), 0x780);
        assert_eq!(offset_of!(StageBattleNintendogs, _0), 0x0);
    }
}
