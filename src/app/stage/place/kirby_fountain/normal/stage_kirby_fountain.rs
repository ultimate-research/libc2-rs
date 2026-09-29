#[repr(C)]
pub struct StageKirbyFountain {
    _0: [u8; 0x8F0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;
        use core::mem::offset_of;

        assert_eq!(size_of::<StageKirbyFountain>(), 0x8F0);
        assert_eq!(offset_of!(StageKirbyFountain, _0), 0x0);
    }
}
