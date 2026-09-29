#[repr(C)]
pub struct StageBattleMarioMaker {
    _0: [u8; 0x800],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleMarioMaker>(), 0x800);
        assert_eq!(offset_of!(StageBattleMarioMaker, _0), 0x0);
    }
}
