#[repr(C)]
pub struct StageBattleKirbyHalberd {
    _0: [u8; 0xB30],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleKirbyHalberd>(), 0xB30);
        assert_eq!(offset_of!(StageBattleKirbyHalberd, _0), 0x0);
    }
}
