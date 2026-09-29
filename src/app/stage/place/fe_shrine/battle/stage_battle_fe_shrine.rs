#[repr(C)]
pub struct StageBattleFEShrine {
    _0: [u8; 0x780],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleFEShrine>(), 0x780);
        assert_eq!(offset_of!(StageBattleFEShrine, _0), 0x0);
    }
}
