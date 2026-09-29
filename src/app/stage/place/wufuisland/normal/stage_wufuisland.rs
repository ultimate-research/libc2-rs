#[repr(C)]
pub struct StageWufuIsland {
    _0: [u8; 0xFB0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageWufuIsland>(), 0xFB0);
        assert_eq!(offset_of!(StageWufuIsland, _0), 0x0);
    }
}
