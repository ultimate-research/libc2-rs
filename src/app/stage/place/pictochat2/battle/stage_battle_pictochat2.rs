#[repr(C)]
pub struct StageBattlePictochat2 {
    _0: [u8; 0xC50],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattlePictochat2>(), 0xC50);
        assert_eq!(offset_of!(StageBattlePictochat2, _0), 0x0);
    }
}
