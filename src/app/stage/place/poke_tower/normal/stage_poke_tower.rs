#[repr(C)]
pub struct StagePokeTower {
    _0: [u8; 0xA50],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StagePokeTower>(), 0xA50);
        assert_eq!(offset_of!(StagePokeTower, _0), 0x0);
    }
}
