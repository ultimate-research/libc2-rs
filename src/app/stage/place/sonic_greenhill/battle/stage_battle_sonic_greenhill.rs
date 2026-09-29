#[repr(C)]
pub struct StageBattleSonicGreenhill {
    _0: [u8; 0x870],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageBattleSonicGreenhill>(), 0x870);
        assert_eq!(offset_of!(StageBattleSonicGreenhill, _0), 0x0);
    }
}
